use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    // Cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
