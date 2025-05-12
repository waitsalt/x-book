use axum::extract::Path;

use crate::{
    module::user::{model::UserPublic, repository},
    util::{AppResult, response::AppResponse},
};

pub async fn info(Path(user_id): Path<i64>) -> AppResult<UserPublic> {
    let user = repository::user_info_get_by_id(&user_id).await?;
    let user_public = UserPublic::from(user);
    Ok(AppResponse::success(Some(user_public)))
}
