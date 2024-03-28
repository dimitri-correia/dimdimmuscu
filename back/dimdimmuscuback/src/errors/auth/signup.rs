use axum::http::StatusCode;

pub enum SignupError {
    ErrorWithDb(libsql::Error),
    ErrorHashing(argon2::password_hash::Error),
    ErrorParsingBirthday,
}

impl SignupError {
    pub fn error_to_show(&self) -> (StatusCode, String) {
        (StatusCode::UNAUTHORIZED, "Signup failed".to_string())
    }
}
