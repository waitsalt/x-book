use axum::Json;

use crate::{
    model::book::{Book, BookSearchPayload},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn search(Json(book_search_payload): Json<BookSearchPayload>) -> AppResult<Vec<Book>> {
    // 搜索
    let sql_search = "
        select * from \"book\"
        where book_name like '%$1%'
        and book_author like '%$2%'
        and book_platform like '%$3%'
        limit $4
        offset $5
        order by book_update desc;
        ";
    let pool = database_connect();
    let book_search_list: Vec<Book> = sqlx::query_as(sql_search)
        .bind(book_search_payload.book_name)
        .bind(book_search_payload.book_author)
        .bind(book_search_payload.book_platform)
        .bind(book_search_payload.limit)
        .bind(book_search_payload.page - 1)
        .fetch_all(pool)
        .await?;
    Ok(AppResponse::success(Some(book_search_list)))
}
