use axum::{Router, routing::get};

use super::service;

pub fn init() -> Router {
    Router::new().route("/", get(service::list))
}
