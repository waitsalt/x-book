use crate::{
    model::user::UserClaim,
    util::{AppResult, response::AppResponse},
};

async fn additon(user_claim: UserClaim) -> AppResult<String> {
    // if user_claim.data.user_id

    Ok(AppResponse::success(Some("asdas".to_string())))
}
