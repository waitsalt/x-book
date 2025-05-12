use axum::Json;

use crate::{
    module::user::{
        model::{UserPublic, UserSearchPayload},
        repository,
    },
    util::{AppResult, response::AppResponse},
};

pub async fn search(
    Json(user_search_payload): Json<UserSearchPayload>,
) -> AppResult<Vec<UserPublic>> {
    let user_list = repository::user_search_by_name(&user_search_payload.keyword).await?;
    let mut user_public_list: Vec<UserPublic> = Vec::new();
    for user in user_list {
        user_public_list.push(UserPublic::from(user));
    }
    Ok(AppResponse::success(Some(user_public_list)))
}
