use axum::Json;

use crate::{
    model::book::{Book, BookSearchPayload},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn search(Json(book_search_payload): Json<BookSearchPayload>) -> AppResult<Vec<Book>> {
    // 搜索
    let pool = database_connect();
    let offset = book_search_payload.page * (book_search_payload.limit - 1);
    let sql = format!(
        "
        select * from \"book\"
        where book_name like '%{}%'
        and book_author like '%{}%'
        and book_platform like '%{}%'
        limit {}
        offset {}
        order by book_update desc;
        ",
        book_search_payload.book_name,
        book_search_payload.author_name,
        book_search_payload.platform_name,
        book_search_payload.limit,
        offset
    );
    let book_search_list: Vec<Book> = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(AppResponse::success(Some(book_search_list)))
}
