use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommentReply {
    pub comment_reply_id: i64,               // 回复 id
    pub comment_status: i16,                 // 评论状态 0:删除 1:正常
    pub user_id: i64,                        // 用户 id
    pub book_id: i64,                        // 书籍 id
    pub comment_id: i64,                     // 父评论 id
    pub comment_content: String,             // 内容
    pub comment_reply_create: DateTime<Utc>, // 创建时间
    pub comment_reply_update: DateTime<Utc>, // 更新时间
}
