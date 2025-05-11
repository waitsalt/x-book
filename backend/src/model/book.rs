use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::roll::RollResponse;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub book_id: i64,                     // 书id
    pub book_name: String,                // 书名
    pub book_desc: String,                // 简介
    pub book_class: String,               // 书分类
    pub book_tag: String,                 // 书标签
    pub book_heat: i64,                   // 书浏览量
    pub book_collect: i64,                // 书收藏量
    pub book_recommend: i64,              // 书推荐量
    pub book_cover_url: String,           // 书封面url
    pub book_word_count: i64,             // 书字数
    pub book_latest_chapter_id: i64,      // 书最新章节id
    pub book_latest_chapter_name: String, // 书最新章节名
    pub book_create_time: DateTime<Utc>,  // 书发布时间
    pub book_update_time: DateTime<Utc>,  // 书更新时间
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookListQuery {
    pub book_class: Option<String>,
    pub book_tag: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSearchQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub book_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookCatalogResponse {
    pub roll_list: Vec<RollResponse>,
}

impl BookCatalogResponse {
    pub fn new() -> Self {
        Self {
            roll_list: Vec::new(),
        }
    }
}
