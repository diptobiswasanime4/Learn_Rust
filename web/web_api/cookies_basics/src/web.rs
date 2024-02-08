use crate::{Error, Result};
use axum::{
    Json,
    Router,
    routing::post,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo1" || payload.password != "pass" {
        return Err(Error::LoginFail)
    }

    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}