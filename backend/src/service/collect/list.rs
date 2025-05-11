use crate::{
    model::{collect::Collect, user::UserClaim},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn list(user_claim: UserClaim) -> AppResult<Collect> {
    let sql = "
        select *
        from \"collect\"
        where user_id = $1
        ";
    let pool = database_connect();
    let collect: Collect = sqlx::query_as(sql)
        .bind(user_claim.data.user_id)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(collect)))
}
