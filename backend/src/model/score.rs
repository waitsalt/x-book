use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub score_id: i64,               // 评分 id
    pub user_id: i64,                // 用户 id
    pub score_book_id_list: String,  // 评分书籍 id 列表
    pub score_create: DateTime<Utc>, // 创建时间
    pub score_update: DateTime<Utc>, // 更新时间
}
