use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use shuttle_common::SecretStore;

use crate::libs::env::init_env;
use crate::libs::routers::auth::auth_routes;
use crate::libs::routers::connected::routes_connected;
use crate::libs::routers::fallback::fallback;

pub async fn main_router(secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let env_variables = init_env(secrets).await;

    let router = Router::new()
        .route("/", get(root))
        .nest("/connect", auth_routes(env_variables.clone()))
        .nest("/api", routes_connected(env_variables))
        .fallback(fallback);

    Ok(router.into())
}

async fn root() -> impl IntoResponse {
    let txt = "Welcome to Dim Dim Muscu!";
    (StatusCode::OK, txt).into_response()
}
