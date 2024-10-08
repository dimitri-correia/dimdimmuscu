use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use log::info;

use crate::libs::env::EnvVariables;

pub fn users_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/get_myself", get(get_myself))
        //.as update
        .with_state(env_variables)
}

async fn get_myself(
    State(_env_variables): State<EnvVariables>,
    req: Request<Body>,
) -> impl IntoResponse {
    let profile_id = req.extensions().get::<String>().unwrap();
    info!("get user info for {}", profile_id);
    (StatusCode::OK, profile_id.to_string()).into_response()
}
