use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Collect {
    pub collect_id: i64,               // 收藏 id
    pub user_id: i64,                  // 用户 id
    pub collect_book_id_list: String,  // 收藏书籍 id 列表
    pub collect_create: DateTime<Utc>, // 创建时间
    pub collect_update: DateTime<Utc>, // 更新时间
}
