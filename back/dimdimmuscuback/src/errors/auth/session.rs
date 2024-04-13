use axum::http::StatusCode;

pub enum SessionError {
    Db(libsql::Error),
    TokenCreation,
    EnvVariableNotSetup,
    FailingParsingEnvVariable,
    TokenDoesntExists,
}

impl SessionError {
    pub fn error_to_show(&self) -> (StatusCode, String) {
        (StatusCode::UNAUTHORIZED, "Auth failed".to_string())
    }
}
