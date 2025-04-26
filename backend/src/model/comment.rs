use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Comment {
    pub comment_id: i64,               // 评论 id
    pub user_id: i64,                  // 用户 id
    pub book_id: i64,                  // 书籍 id
    pub comment_content: String,       // 内容
    pub create: DateTime<Utc>,         // 创建时间
    pub update: DateTime<Utc>,         // 更新时间
    pub like: i32,                     // 点赞数
    pub dislike: i32,                  // 踩数
    pub comment_create: DateTime<Utc>, // 创建时间
    pub comment_update: DateTime<Utc>, // 更新时间
}
