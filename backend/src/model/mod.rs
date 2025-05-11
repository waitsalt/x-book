use serde::{Deserialize, Serialize};

pub mod book;
pub mod chapter;
pub mod chapter_content;
pub mod collect;
pub mod comment;
pub mod comment_reply;
pub mod history;
pub mod roll;
pub mod user;
pub mod util;

#[derive(Debug, Deserialize, Serialize)]
pub struct SqlQueryResultListWithCount<T> {
    pub list: Vec<T>,
    pub count: i64,
}

impl<T> SqlQueryResultListWithCount<T> {
    pub fn new(list: Vec<T>, count: i64) -> Self {
        Self { list, count }
    }
}
