use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub history_id: i64,                    // 历史id
    pub user_id: i64,                       // 用户id
    pub book_id_list: String,               // 历史书id集合
    pub history_create_time: DateTime<Utc>, // 历史发布时间
    pub history_update_time: DateTime<Utc>, // 历史更新时间
}
