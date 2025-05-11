use axum::{Router, routing::get};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::history::list))
        .route("/addition/{book_id}", get(service::history::addition)) // 章节内容
        .route("/cancel/{book_id}", get(service::history::cancel)) // 章节内容
}
