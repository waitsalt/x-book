use crate::{
    model::book::Book,
    util::{AppResult, database::database_connect, response::AppResponse},
};
use axum::extract::Path;

pub async fn info(Path(book_id): Path<i64>) -> AppResult<Book> {
    let sql = "
        select *
        from \"book\"
        where book_id = $1
        ";
    let pool = database_connect();
    let book: Book = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(book)))
}
