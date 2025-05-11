use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommentReply {
    pub comment_id: i64,                          // 父评论
    pub comment_reply_id: i64,                    // 回复评论id
    pub user_id: i64,                             // 用户id
    pub comment_reply_content: String,            // 评论内容
    pub comment_reply_praise: i64,                // 评论点赞
    pub comment_reply_create_time: DateTime<Utc>, // 回复评论发布时间
    pub comment_reply_update_time: DateTime<Utc>, // 回复评论更新时间
}
