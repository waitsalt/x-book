use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Roll {
    pub roll_id: i64,               // 卷 id
    pub roll_name: String,          // 卷名称
    pub book_id: i64,               // 书籍 id
    pub book_name: String,          // 书籍名称
    pub roll_create: DateTime<Utc>, // 创建时间
    pub roll_update: DateTime<Utc>, // 更新时间
}
