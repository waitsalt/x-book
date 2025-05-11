use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Collect {
    pub collect_id: i64,                    // 收藏id
    pub collect_name: String,               // 收藏名
    pub user_id: i64,                       // 用户id
    pub user_name: i64,                     // 用户名
    pub book_id_list: String,               // 收藏书id集合
    pub collect_create_time: DateTime<Utc>, // 卷发布时间
    pub collect_update_time: DateTime<Utc>, // 卷更新时间
}
