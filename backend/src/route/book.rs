use axum::{
    Router,
    routing::{get, post},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", get(service::book::list))
        .route("/{book_id}", get(service::book::info))
        .route("/search", post(service::book::search))
}
