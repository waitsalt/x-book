use axum::extract::Path;

use crate::{
    model::chapter_content::ChapterContent,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn content(Path(chapter_id): Path<i64>) -> AppResult<ChapterContent> {
    let pool = database_connect();
    let sql = format!(
        "
        select *
        from \"chapter_content\"
        where chapter_content_id = {}
        ",
        chapter_id
    );
    let chapter_content: ChapterContent = sqlx::query_as(&sql).fetch_one(pool).await?;

    Ok(AppResponse::success(Some(chapter_content)))
}
