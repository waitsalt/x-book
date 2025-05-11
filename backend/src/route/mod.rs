mod book;
mod chapter_content;
mod collect;
mod history;
mod user;
mod util;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    // 获取路由
    let book_router = book::init();
    let chapter_content_router = chapter_content::init();
    let collect_router = collect::init();
    let history_router = history::init();
    let user_router = user::init();
    let util_router = util::init();

    // 设置请求允许
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 加载路由
    Router::new()
        .nest("/api/v0/book", book_router)
        .nest("/api/v0/chapter_content", chapter_content_router)
        .nest("/api/v0/collect", collect_router)
        .nest("/api/v0/user", user_router)
        .nest("/api/v0/util", util_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
