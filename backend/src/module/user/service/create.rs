use axum::Json;
use redis::Commands;

use crate::{
    module::user::{model::UserCreatePayload, repository},
    util::{AppResult, error::AppError, redis::redis_connect, response::AppResponse},
};

pub async fn create(Json(user_create_payload): Json<UserCreatePayload>) -> AppResult<()> {
    // 检查密码
    if user_create_payload.user_password.len() < 8 {
        return Err(AppError::UserPasswordShort);
    }

    // 查询用户名是否存在
    repository::user_name_is_exist(&user_create_payload.user_name).await?;

    // 查询用户邮箱是否存在
    repository::user_email_is_exist(&user_create_payload.user_email).await?;

    // 验证邮箱验证码
    let mut con = redis_connect();

    let captcha_email_key = format!("captcha_email_key:{}", user_create_payload.user_email);
    let captcha_email_value: String = con.get(&captcha_email_key)?;

    if captcha_email_value != user_create_payload.captcha_email {
        return Err(AppError::CaptchaEmailValueError);
    }

    // 邮箱验证码使用后失效
    let _: () = con.del(&captcha_email_key)?;

    // 新建用户
    repository::user_create(
        &user_create_payload.user_name,
        &user_create_payload.user_password,
        &user_create_payload.user_email,
        &user_create_payload.user_avatar_url,
    )
    .await?;

    Ok(AppResponse::success(None))
}
