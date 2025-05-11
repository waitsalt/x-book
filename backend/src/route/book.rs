use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::book::list)) // 列表
        .route("/search", get(service::book::search)) // 搜索
        .route("/{book_id}", get(service::book::info)) // 书信息
        .route("/{book_id}/catalog", get(service::book::catalog)) // 书目录
        .route("/{book_id}/comment", get(service::book::comment)) // 书评论
}
