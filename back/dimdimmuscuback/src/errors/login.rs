use axum::http::StatusCode;

pub enum LoginError {
    LoginFailUsernameNotFound,
    LoginFailPwdNotMatching { user_id: i32 },
    SetCookieFailed,
    FailUpdate,
}

impl LoginError {
    pub fn login_failed(&self) -> (StatusCode, String) {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string())
    }
}
