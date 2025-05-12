use axum::Json;

use crate::{
    module::user::{
        model::{UserChangePasswordPayload, UserClaim},
        repository,
    },
    util::{AppResult, error::AppError, response::AppResponse},
};

pub async fn change_password(
    user_claim: UserClaim,
    Json(user_change_password_payload): Json<UserChangePasswordPayload>,
) -> AppResult<()> {
    let user = repository::user_info_get_by_id(&user_claim.data.user_id).await?;
    if user.user_password != user_change_password_payload.old {
        return Err(AppError::UserPasswordError);
    }
    repository::update_password(&user_claim.data.user_id, &user_change_password_payload.new)
        .await?;
    Ok(AppResponse::success(None))
}
