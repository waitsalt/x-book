use axum::{Router, routing::get};

use super::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::list)) // 列表
        .route("/search", get(service::search)) // 搜索
        .route("/{book_id}", get(service::info)) // 书信息
        .route("/{book_id}/catalog", get(service::catalog)) // 书目录
        .route("/{book_id}/comment", get(service::comment)) // 书评论
}
