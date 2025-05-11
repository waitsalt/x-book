use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::collect::list))
        .route("/addition/{book_id}", get(service::collect::addition)) // 章节内容
        .route("/cancel/{book_id}", get(service::collect::cancel)) // 章节内容
}
