use axum::extract::Path;

use crate::{
    model::{chapter::Chapter, roll::Roll},
    util::AppResult,
};

struct RollResponse {
    pub roll_info: Roll,
    pub roll_chapter_list: Vec<Chapter>,
}
pub async fn chapter_list(Path(book_id): Path<i64>) -> AppResult<Vec<RollResponse>> {
    let pool
    let roll_list: Vec<Roll> = sqlx::query_as(sql)
}
