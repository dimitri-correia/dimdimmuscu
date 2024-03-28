use axum::Router;

use crate::env::init_env;
use crate::routers::auth::auth_routes;
use crate::routers::fallback::fallback;

mod db;
mod env;
mod errors;
mod logic;
mod mw;
mod routers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let env_variables = init_env().await;

    let router = Router::new()
        .nest("/connect", auth_routes(env_variables.clone()))
        // .nest("/api", routes_connected())
        .fallback(fallback);
    //.layer(CookieManagerLayer::new());

    Ok(router.into())
}

// fn routes_connected() -> Router {
//     todo!()
// }
