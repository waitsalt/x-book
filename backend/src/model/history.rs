use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct History {
    pub history_id: i32,               // 历史记录ID
    pub user_id: i32,                  // 用户ID
    pub history_book_id_list: String,  // 阅读书籍ID列表
    pub history_create: DateTime<Utc>, // 创建时间
    pub history_update: DateTime<Utc>, // 更新时间
}
