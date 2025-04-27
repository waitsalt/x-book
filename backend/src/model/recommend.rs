use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Recommend {
    pub recommend_id: i64,               // 推荐 id
    pub user_id: i64,                    // 用户 id
    pub recommend_book_id_list: String,  // 推荐书籍 id 列表
    pub recommend_create: DateTime<Utc>, // 创建时间
    pub recommend_update: DateTime<Utc>, // 更新时间
}
