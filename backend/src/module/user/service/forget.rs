use axum::Json;
use redis::Commands;

use crate::{
    module::user::model::{User, UserForgetPayload},
    util::{
        AppResult, database::database_connect, error::AppError, redis::redis_connect,
        response::AppResponse,
    },
};

pub async fn forget(Json(user_forget_payload): Json<UserForgetPayload>) -> AppResult<()> {
    let sql = "
        select *
        from \"user\"
        where user_email = $1
        ";
    let pool = database_connect();
    let user: User = sqlx::query_as(sql)
        .bind(&user_forget_payload.user_email)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::UserNotExist)?;
    let mut con = redis_connect();
    let captcha_email: String = con
        .get(format!(
            "captcha_email_key:{}",
            &user_forget_payload.user_email
        ))
        .map_err(|_| AppError::CaptchaEmailValueNotExist)?;
    if captcha_email != user_forget_payload.captcha_email {
        return Err(AppError::CaptchaEmailValueError);
    }
    let sql = "
        update \"user\"
        set user_password = $1
        where user_id = $2
        ";
    let _ = sqlx::query(sql)
        .bind(user_forget_payload.user_password)
        .bind(user.user_id)
        .execute(pool)
        .await
        .unwrap();

    Ok(AppResponse::success(None))
}
