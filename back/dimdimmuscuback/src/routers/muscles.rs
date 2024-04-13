use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use crate::db::structs::users_auth::UserForCreate;
use crate::env::EnvVariables;
use crate::mw::mw_auth::SessionToken;

pub fn muscles_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/signup", post(add_new))
        .with_state(env_variables)
}

async fn add_new(
    token: SessionToken,
    State(env_variables): State<EnvVariables>,
    Json(user): Json<UserForCreate>,
) -> impl IntoResponse {
    if let Some(creation) = user
        .add_new_user_in_db(&env_variables.db_connection)
        .await
        .err()
    {
        return creation.into_response();
    };

    // we choose to not give a cookie here but to force authentication with /login endpoint
    (StatusCode::CREATED, "User creation worked".to_string()).into_response()
}
