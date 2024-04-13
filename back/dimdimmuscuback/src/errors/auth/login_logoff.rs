use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum LoginError {
    ErrorWithDb(libsql::Error),
    ErrorWithHash,
    WrongPwd,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string()).into_response()
    }
}

pub enum LogoffError {
    ErrorWithDb(libsql::Error),
    NotConnected,
}

impl IntoResponse for LogoffError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Deletion failed".to_string()).into_response()
    }
}
