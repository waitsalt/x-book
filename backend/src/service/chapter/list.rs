use axum::extract::Path;
use serde::{Deserialize, Serialize};

use crate::{
    model::{chapter::Chapter, roll::Roll},
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

#[derive(Deserialize, Serialize)]
pub struct RollResponse {
    pub roll_info: Roll,
    pub roll_chapter_list: Vec<Chapter>,
}

pub async fn list(Path(book_id): Path<i64>) -> AppResult<Vec<RollResponse>> {
    let pool = database_connect();
    let sql = "
        select *
        from roll
        where book_id = $1
        ";
    let roll_list: Vec<Roll> = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::BookNotExist)?;

    let mut roll_response_list: Vec<RollResponse> = Vec::new();

    for roll in roll_list {
        let sql = "
            select *
            from chapter
            where roll_id = $1
            ";
        let chapter_list: Vec<Chapter> = sqlx::query_as(sql)
            .bind(roll.roll_id)
            .fetch_all(pool)
            .await?;
        let roll_response = RollResponse {
            roll_info: roll,
            roll_chapter_list: chapter_list,
        };
        roll_response_list.push(roll_response);
    }

    Ok(AppResponse::success(Some(roll_response_list)))
}
