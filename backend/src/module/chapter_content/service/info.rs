use axum::extract::Path;

use crate::{
    module::chapter_content::model::ChapterContent,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn info(Path(chapter_id): Path<i64>) -> AppResult<ChapterContent> {
    let sql = "
        select *
        from \"chapter_content\"
        where chapter_id = $1
        ";
    let pool = database_connect();
    let chapter_content: ChapterContent = sqlx::query_as(sql)
        .bind(chapter_id)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(chapter_content)))
}
