use crate::util::{AppResult, response::AppResponse};

pub async fn forget() -> AppResult<()> {
    Ok(AppResponse::success(None))
}
