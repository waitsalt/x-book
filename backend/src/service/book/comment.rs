use axum::extract::Path;

use crate::{
    model::comment::Comment,
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn comment(Path(book_id): Path<i64>) -> AppResult<Vec<Comment>> {
    let pool = database_connect();
    let sql = "
        select *
        from \"comment\"
        where book_id = $1
        and comment_reply_is = false
        order by comment_id
        ";
    let comment_list: Vec<Comment> = sqlx::query_as(sql)
        .bind(book_id)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(comment_list)))
}
