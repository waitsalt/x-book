use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub comment_id: i64,                    // 评论id
    pub user_id: i64,                       // 用户id
    pub book_id: i64,                       // 图书id
    pub comment_content: String,            // 评论内容
    pub comment_praise: i64,                // 评论点赞
    pub comment_create_time: DateTime<Utc>, // 评论发布时间
    pub comment_update_time: DateTime<Utc>, // 评论更新时间
}
