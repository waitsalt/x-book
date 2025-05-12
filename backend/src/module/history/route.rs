use axum::{Router, routing::get};

use super::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::list))
        .route("/addition/{book_id}", get(service::addition)) // 章节内容
        .route("/cancel/{book_id}", get(service::cancel)) // 章节内容
}
