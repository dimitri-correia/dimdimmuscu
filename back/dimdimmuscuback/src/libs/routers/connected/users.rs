use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use log::info;

use crate::libs::env::EnvVariables;
use crate::libs::mw::mw_auth::SessionToken;

pub fn users_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/get_myself", get(get_myself))
        .with_state(env_variables)
}

async fn get_myself(
    token: SessionToken,
    State(_env_variables): State<EnvVariables>,
) -> impl IntoResponse {
    info!("get user info for {}", token.profile_id);
    (StatusCode::OK, "".to_string()).into_response()
}
