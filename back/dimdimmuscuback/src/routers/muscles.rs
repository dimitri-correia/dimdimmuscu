use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::info;

use crate::db::structs::users_auth::UserForCreate;
use crate::env::EnvVariables;
use crate::mw::mw_auth::SessionToken;

pub fn muscles_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/add", post(add_new))
        .with_state(env_variables)
}

async fn add_new(
    token: SessionToken,
    State(env_variables): State<EnvVariables>,
    Json(muscle): Json<UserForCreate>,
) -> impl IntoResponse {
    info!("{} is adding {}", token.profile_id, muscle.username);
    (StatusCode::CREATED, "User creation worked".to_string()).into_response()
}
