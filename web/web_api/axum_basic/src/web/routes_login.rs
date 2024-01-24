use crate::{Error, Result};
use axum::{
    Json,
    routing::post,
    Router
};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie,Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

pub fn routes() -> Router {
    Router::new().route("/api_login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("api_login");

    if payload.username != "demo" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}