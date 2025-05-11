use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::chapter::Chapter;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Roll {
    pub roll_id: i64,                    // 卷id
    pub roll_name: String,               // 卷名
    pub book_id: i64,                    // 书id
    pub book_name: String,               // 书名
    pub roll_create_time: DateTime<Utc>, // 卷发布时间
    pub roll_update_time: DateTime<Utc>, // 卷更新时间
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollResponse {
    pub roll: Roll,
    pub chapter_list: Vec<Chapter>,
}

impl RollResponse {
    pub fn new(roll: Roll, chapter_list: Vec<Chapter>) -> Self {
        Self { roll, chapter_list }
    }
}
