use axum::Router;

use crate::libs::env::EnvVariables;
use crate::libs::routers::connected::muscles::muscles_routes;

pub mod muscles;
pub mod users;

pub fn routes_connected(env_variables: EnvVariables) -> Router {
    Router::new().nest("/muscles", muscles_routes(env_variables))
}
