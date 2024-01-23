use crate::{Error, Result};
use axum::{
    Json,
    routing::post,
    Router
};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

pub fn routes() -> Router {
    Router::new().route("/api_login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("api_login");

    if payload.username != "demo" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}