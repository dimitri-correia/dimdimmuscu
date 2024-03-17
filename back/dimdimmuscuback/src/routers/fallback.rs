use axum::http::StatusCode;

pub async fn fallback() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Error".to_string())
}
