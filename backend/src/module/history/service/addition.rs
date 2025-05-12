use axum::extract::Path;
use chrono::Utc;

use crate::{
    module::{
        book::model::Book,
        history::model::History,
        user::model::{User, UserClaim},
    },
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

pub async fn addition(user_claim: UserClaim, Path(book_id): Path<i64>) -> AppResult<()> {
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
        .map_err(|_| AppError::BookNotExist)?;
    let sql = "
        select *
        from \"user\"
        where user_id = $1
        ";
    let _user: User = sqlx::query_as(sql)
        .bind(&user_claim.data.user_id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::UserNotExist)?;
    let sql = "
        select *
        from \"history\"
        where user_id = $1
        ";
    let result: Result<History, sqlx::Error> = sqlx::query_as(sql)
        .bind(&user_claim.data.user_id)
        .fetch_one(pool)
        .await;
    match result {
        Ok(history) => {
            let mut book_id_list: Vec<i64> = serde_json::from_str(&history.book_id_list).unwrap();
            if book_id_list.contains(&book.book_id) {
                return Err(AppError::HistoryBookExist);
            }
            book_id_list.push(book.book_id);
            let book_id_list_string = serde_json::to_string(&book_id_list).unwrap();
            let history_update_time = Utc::now();
            let sql = "
                update \"history\"
                set book_id_list = $1
                and history_update_time = $2
                where history_id = $3
                ";
            let _ = sqlx::query(sql)
                .bind(book_id_list_string)
                .bind(history_update_time)
                .bind(history.history_id)
                .execute(pool)
                .await
                .unwrap();
        }
        Err(_) => {
            let sql = "
                insert into \"history\" (user_id, book_id_list)
                values ($1, $2)
                ";
            let mut book_id_list = Vec::new();
            book_id_list.push(book.book_id);
            let book_id_list_string: String = serde_json::to_string(&book_id_list).unwrap();
            let _ = sqlx::query(sql)
                .bind(&user_claim.data.user_id)
                .bind(book_id_list_string)
                .execute(pool)
                .await
                .unwrap();
        }
    }

    Ok(AppResponse::success(None))
}
