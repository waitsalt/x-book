use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new().route("/{chapter_id}", get(service::chapter_content::info)) // 章节内容
}
