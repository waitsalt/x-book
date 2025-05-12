use crate::{
    module::{history::model::History, user::model::UserClaim},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn list(user_claim: UserClaim) -> AppResult<History> {
    let sql = "
        select *
        from \"history\"
        where user_id = $1
        ";
    let pool = database_connect();
    let history: History = sqlx::query_as(sql)
        .bind(user_claim.data.user_id)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(history)))
}
