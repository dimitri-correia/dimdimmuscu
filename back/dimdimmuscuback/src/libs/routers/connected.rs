use axum::{middleware, Router};

use crate::libs::env::EnvVariables;
use crate::libs::mw::mw_auth::mw_auth;

pub mod muscles;
pub mod users;

pub(super) fn routes_connected(env_variables: EnvVariables) -> Router {
    Router::new()
        .nest("/users", users::users_routes(env_variables.clone()))
        .nest("/muscles", muscles::muscles_routes(env_variables.clone()))
        .route_layer(middleware::from_fn(mw_auth))
}
