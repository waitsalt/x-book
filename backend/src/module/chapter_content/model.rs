use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ChapterContent {
    pub chapter_content_id: i64,                    // 章内容id
    pub chapter_id: i64,                            // 章id
    pub chapter_name: String,                       // 章名
    pub chapter_content: String,                    // 章内容
    pub roll_id: i64,                               // 卷id
    pub roll_name: String,                          // 卷名
    pub book_id: i64,                               // 书id
    pub book_name: String,                          // 书名
    pub chapter_content_create_time: DateTime<Utc>, // 章发布时间
    pub chapter_content_update_time: DateTime<Utc>, // 章更新时间
}
