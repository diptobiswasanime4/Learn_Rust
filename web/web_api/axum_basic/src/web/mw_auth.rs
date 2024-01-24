use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::{
    response::Response,
    extract::Request,
    middleware::Next,
    body::Body
};
use tower_cookies::Cookies;


pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
