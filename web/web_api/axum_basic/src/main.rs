use crate::model::ModelController;
use tokio::net::TcpListener;
use axum::{
    routing::{get, get_service},
    Router,
    middleware,
    response::Response,
    response::{Html, IntoResponse},
    extract::{Query, Path}
};
use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

mod web;
mod error;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let app = Router::new()
       .merge(routes_hello())
       .merge(web::routes_login::routes())
       .nest("/api", web::routes_tickets::routes(mc.clone()))
       .layer(middleware::map_response(main_response_mapper))
       .layer(CookieManagerLayer::new())
       .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("App listening on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("");
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_name))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {    
    println!("handler_hello {:?}", params);

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("<h1>Hello World {name}</h1>"))
}

async fn handler_name(Path(name): Path<String>) -> impl IntoResponse {
    println!("handler_hello {:?}", name);

    Html(format!("<h1>Hello {name}</h1>"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}