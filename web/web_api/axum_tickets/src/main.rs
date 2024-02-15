use axum::{
    extract::{Path, State},
    http::Method,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use tokio::net::TcpListener;

mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let mc = ModelController::new().await?;

    let app = Router::new().merge(web::routes_login::routes());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("App running on Port {:?}", listener.local_addr());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
