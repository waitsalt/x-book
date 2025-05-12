use axum::extract::Path;

use crate::{
    module::user::{model::UserClaim, repository},
    util::{AppResult, error::AppError, response::AppResponse},
};

pub async fn delete(user_claim: UserClaim, Path(user_id): Path<i64>) -> AppResult<()> {
    if user_claim.data.user_id != user_id {
        return Err(AppError::PermissionDenied);
    }
    repository::user_delete(&user_id).await?;
    Ok(AppResponse::success(None))
}
