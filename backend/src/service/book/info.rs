use axum::extract::Path;

use crate::{
    model::book::Book,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

pub async fn info(Path(book_id): Path<i64>) -> AppResult<Book> {
    let pool = database_connect();
    let sql = "
        select *
        form \"book\"
        where book_id = $1;
        ";
    let book: Book = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::BookNotExist)?;
    Ok(AppResponse::success(Some(book)))
}
