use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum SessionError {
    Db(libsql::Error),
    TokenCreation,
    EnvVariableNotSetup,
    FailingParsingEnvVariable,
    TokenDoesntExists,
    BadToken,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string()).into_response()
    }
}
