use axum::extract::Query;

use crate::{
    module::{
        book::model::{Book, BookListQuery},
        model::SqlQueryResultListWithCount,
    },
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn list(
    Query(book_list_query): Query<BookListQuery>,
) -> AppResult<SqlQueryResultListWithCount<Book>> {
    let mut condition_sql = String::new();
    // 判断是否要加 book_class 进行查询
    let book_class = match book_list_query.book_class {
        Some(book_class) => {
            condition_sql += " and book_class like '%$1%'";
            book_class
        }
        None => "".to_string(),
    };

    // 判断是否要加 book_tag 进行查询
    let book_tag = match book_list_query.book_tag {
        Some(book_tag) => {
            condition_sql += " and book_tag like '%$2%'";
            book_tag
        }
        None => "".to_string(),
    };

    let count_sql = String::from("select count(*) from \"book\" where 1 = 1 ") + &condition_sql;

    // 判断是否要加 limit 进行查询
    let limit = match book_list_query.limit {
        Some(limit) => {
            condition_sql += " limit $3";
            limit
        }
        None => {
            condition_sql += " limit $3";
            10
        }
    };

    // 判断是否要加 page 进行查询
    let offset = match book_list_query.page {
        Some(page) => {
            condition_sql += " offset $4";
            (page - 1) * limit
        }
        None => 0,
    };
    let list_sql = String::from("select * from \"book\" where 1 = 1") + &condition_sql;

    let pool = database_connect();
    let count: i64 = sqlx::query_scalar(&count_sql)
        .bind(&book_class)
        .bind(&book_tag)
        .fetch_one(pool)
        .await
        .unwrap();
    let book_list: Vec<Book> = sqlx::query_as(&list_sql)
        .bind(&book_class)
        .bind(&book_tag)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap();

    let result = SqlQueryResultListWithCount::new(count, book_list);

    Ok(AppResponse::success(Some(result)))
}
