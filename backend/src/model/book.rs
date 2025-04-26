use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Book {
    pub book_id: i64,                     // 书 id
    pub book_name: String,                // 书名称
    pub author_id: i64,                   // 作者 id
    pub author_name: String,              // 作者名称
    pub platform_id: i64,                 // 平台 id
    pub platform_name: String,            // 平台名称
    pub book_cover_url: String,           // 书籍封面 URL
    pub book_desc: String,                // 书籍描述
    pub book_tag: String,                 // 书籍标签
    pub book_class: String,               // 书籍分类
    pub book_status: i16,                 // 书籍状态 0:未开始 1:连载中 2:已完结 3:断更
    pub book_latest_chapter_id: i64,      // 最后一章 id
    pub book_latest_chapter_name: String, // 最后一章名称
    pub book_publisher_id: i64,           // 发布用户 id
    pub book_publisher_name: String,      // 发布用户名称
    pub book_read: i64,                   // 阅读量
    pub book_collect: i64,                // 收藏量
    pub book_recommend: i64,              // 推荐量
    pub book_create: DateTime<Utc>,       // 创建时间
    pub book_update: DateTime<Utc>,       // 更新时间
}

#[derive(Debug, Deserialize)]
pub struct BookSearchPayload {
    pub book_name: String,
    pub author_name: String,
    pub platform_name: String,
    pub limit: i64,
    pub page: i64,
    pub tag: Vec<String>,
    pub class: String,
}
