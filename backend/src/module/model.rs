use serde::{Deserialize, Serialize};

use crate::util::{error::AppError, response::AppResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct SqlQueryResultListWithCount<T> {
    count: i64,
    list: Vec<T>,
}

impl<T> SqlQueryResultListWithCount<T> {
    pub fn new(count: i64, list: Vec<T>) -> Self {
        Self { count, list }
    }
}

pub type SqlResult<T> = Result<T, AppError>;

pub type AppResult<T> = std::result::Result<AppResponse<T>, AppError>;
