use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    TicketDeleteFailNotFound { id: u64 }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}