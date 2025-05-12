use axum::{Router, routing::get};

use super::service;

pub fn init() -> Router {
    Router::new().route("/{chapter_id}", get(service::info)) // 章节内容
}
