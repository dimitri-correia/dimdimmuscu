use axum::http::StatusCode;

pub enum LoginError {
    ErrorWithDb(libsql::Error),
    ErrorWithHash,
    WrongPwd,
}

impl LoginError {
    pub fn error_to_show(&self) -> (StatusCode, String) {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string())
    }
}
