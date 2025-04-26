use axum::extract::Query;
use serde::Deserialize;

use crate::{
    model::{SqlQueryResponseListAndCount, book::Book},
    util::{AppResult, database::database_connect, response::AppResponse},
};

#[derive(Deserialize)]
pub struct Test {
    pub limit: i64,
    pub page: i64,
}

pub async fn list(Query(test): Query<Test>) -> AppResult<SqlQueryResponseListAndCount<Book>> {
    let pool = database_connect();

    // 查询数量
    let sql = "
        select count(*)
        from \"book\"
        ";
    let count = sqlx::query_scalar(sql).fetch_one(pool).await?;

    // 查询列表
    let offset = (test.page - 1) * test.limit;
    let sql = format!(
        "
        select *
        from \"book\"
        limit {}
        offset {}
        ",
        test.limit, offset
    );
    let book_list = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(AppResponse::success(Some(SqlQueryResponseListAndCount {
        count,
        list: book_list,
    })))
}
