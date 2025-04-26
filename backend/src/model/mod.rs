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
pub struct SqlQueryResponseListAndCount<T> {
    pub count: i64,
    pub list: Vec<T>,
}
