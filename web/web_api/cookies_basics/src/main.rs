use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_hello))
        .merge(web::routes())
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler_hello() -> &'static str {
    "Hello"
}
