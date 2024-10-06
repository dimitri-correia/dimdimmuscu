use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum SignupError {
    ErrorWithDb(libsql::Error),
    ErrorHashing(argon2::password_hash::Error),
    ErrorParsingBirthday,
    HeightOutOfRange,
}

impl IntoResponse for SignupError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Signup failed".to_string()).into_response()
    }
}
