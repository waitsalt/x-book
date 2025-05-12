use super::*;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    // 获取路由
    let book_router = book::route::init();
    let captcha_router = captcha::route::init();
    let chapter_content_router = chapter_content::route::init();
    let collect_router = collect::route::init();
    let comment_router = comment::route::init();
    let history_router = history::route::init();
    let user_router = user::route::init();

    // 设置请求允许
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 加载路由
    Router::new()
        .nest("/api/v0/book", book_router)
        .nest("/api/v0/captcha", captcha_router)
        .nest("/api/v0/chapter_content", chapter_content_router)
        .nest("/api/v0/collect", collect_router)
        .nest("/api/v0/comment", comment_router)
        .nest("/api/v0/history", history_router)
        .nest("/api/v0/user", user_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
