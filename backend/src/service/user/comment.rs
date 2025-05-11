use crate::{
    model::{comment::Comment, user::UserClaim},
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn comment(user_claim: UserClaim) -> AppResult<Vec<Comment>> {
    let pool = database_connect();
    let sql = "
        select *
        from \"comment\"
        where user_id = $1
        ";
    let comment_list: Vec<Comment> = sqlx::query_as(sql)
        .bind(&user_claim.data.user_id)
        .fetch_all(pool)
        .await
        .unwrap();
    Ok(AppResponse::success(Some(comment_list)))
}
