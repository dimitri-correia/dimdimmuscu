use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use log::info;

use crate::libs::env::EnvVariables;

pub fn muscles_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/get", get(get_one))
        .with_state(env_variables)
}

async fn get_one(
    //_token: SessionToken,
    State(_env_variables): State<EnvVariables>,
    // Json(muscle): Json<UserForCreate>,
) -> impl IntoResponse {
    info!("{} is adding {}", "token.profile_id", "muscle.username");
    (StatusCode::CREATED, "Muscle creation worked".to_string()).into_response()
}
