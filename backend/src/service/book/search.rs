use axum::extract::Query;

use crate::{
    model::{
        SqlQueryResultListWithCount,
        book::{Book, BookSearchQuery},
    },
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn search(
    Query(book_search_query): Query<BookSearchQuery>,
) -> AppResult<SqlQueryResultListWithCount<Book>> {
    let mut condition_sql = String::new();
    // 判断是否要加 book_class 进行查询
    let book_name = match book_search_query.book_name {
        Some(book_name) => {
            condition_sql += " and book_name like '%$1%'";
            book_name
        }
        None => "".to_string(),
    };

    let count_sql = format!(
        "select count(*) from \"book\" where 1 = 1 {}",
        condition_sql
    );

    // 判断是否要加 limit 进行查询
    let limit = match book_search_query.limit {
        Some(limit) => {
            condition_sql += " limit $2";
            limit
        }
        None => {
            condition_sql += " limit $2";
            10
        }
    };

    // 判断是否要加 page 进行查询
    let offset = match book_search_query.page {
        Some(page) => {
            condition_sql += " offset $3";
            (page - 1) * limit
        }
        None => 0,
    };
    let list_sql = String::from("select * from user where 1 = 1") + &condition_sql;

    let pool = database_connect();
    let count: i64 = sqlx::query_scalar(&count_sql)
        .bind(&book_name)
        .fetch_one(pool)
        .await
        .unwrap();
    let book_list: Vec<Book> = sqlx::query_as(&list_sql)
        .bind(&book_name)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap();

    let result = SqlQueryResultListWithCount::new(book_list, count);

    Ok(AppResponse::success(Some(result)))
}
