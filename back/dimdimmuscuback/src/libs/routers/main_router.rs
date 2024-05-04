use axum::Router;

use crate::libs::env::init_env;
use crate::libs::routers::auth::auth_routes;
use crate::libs::routers::connected::routes_connected;
use crate::libs::routers::fallback::fallback;

pub async fn main_router() -> shuttle_axum::ShuttleAxum {
    let env_variables = init_env().await;

    let router = Router::new()
        .nest("/connect", auth_routes(env_variables.clone()))
        .nest("/api", routes_connected(env_variables))
        .fallback(fallback);

    Ok(router.into())
}
