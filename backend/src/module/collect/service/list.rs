use crate::{
    module::{collect::model::Collect, user::model::UserClaim},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn list(user_claim: UserClaim) -> AppResult<Vec<Collect>> {
    let sql = "
        select *
        from \"collect\"
        where user_id = $1
        ";
    let pool = database_connect();
    let collect: Vec<Collect> = sqlx::query_as(sql)
        .bind(user_claim.data.user_id)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(collect)))
}
