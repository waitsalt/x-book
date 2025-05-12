use axum::Json;

use crate::{
    module::user::{
        model::{UserChangeAvatarUrlPayload, UserClaim},
        repository,
    },
    util::{AppResult, response::AppResponse},
};

pub async fn change_avatar_url(
    user_claim: UserClaim,
    Json(user_change_avatar_url_payload): Json<UserChangeAvatarUrlPayload>,
) -> AppResult<()> {
    repository::update_avatar_url(
        &user_claim.data.user_id,
        &user_change_avatar_url_payload.new,
    )
    .await?;
    Ok(AppResponse::success(None))
}
