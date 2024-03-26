use axum::http::StatusCode;

pub enum CookieError {
    ErrorWithDb(sqlx::Error),
}

impl CookieError {
    pub fn error_to_show(&self) -> (StatusCode, String) {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string())
    }
}
