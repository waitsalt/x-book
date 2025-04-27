use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ChapterContent {
    pub chapter_id: i64,                       // 章节 id
    pub chapter_content: String,               // 章节内容
    pub chapter_name: String,                  // 章节名称
    pub roll_id: i64,                          // 卷 id
    pub roll_name: String,                     // 卷名称
    pub book_id: i64,                          // 书籍 id
    pub book_name: String,                     // 书籍名称
    pub chapter_content_create: DateTime<Utc>, // 创建时间
    pub chapter_content_update: DateTime<Utc>, // 更新时间
}
