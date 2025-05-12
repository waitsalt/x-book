use axum::extract::Path;
use chrono::Utc;

use crate::{
    module::{
        book::model::Book,
        collect::model::Collect,
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
        from \"collect\"
        where user_id = $1
        ";
    let result: Result<Collect, sqlx::Error> = sqlx::query_as(sql)
        .bind(&user_claim.data.user_id)
        .fetch_one(pool)
        .await;
    match result {
        Ok(collect) => {
            let mut book_id_list: Vec<i64> = serde_json::from_str(&collect.book_id_list).unwrap();
            if book_id_list.contains(&book.book_id) {
                return Err(AppError::CollectBookExist);
            }
            book_id_list.push(book.book_id);
            let book_id_list_string = serde_json::to_string(&book_id_list).unwrap();
            let collect_update_time = Utc::now();
            let sql = "
                update \"collect\"
                set book_id_list = $1
                and collect_update_time = $2
                where collect_id = $3
                ";
            let _ = sqlx::query(sql)
                .bind(book_id_list_string)
                .bind(collect_update_time)
                .bind(collect.collect_id)
                .execute(pool)
                .await
                .unwrap();
        }
        Err(_) => {
            let sql = "
                insert into \"collect\" (user_id, book_id_list)
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
