use axum::Router;

use crate::db::methods::init_db::init_db;
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
    let (env_variables, env_variables_db) = init_env();
    let db = init_db(env_variables_db).await;

    let router = Router::new()
        .nest("/connect", auth_routes(db.clone()))
        // .nest("/api", routes_connected())
        .fallback(fallback);
    //.layer(CookieManagerLayer::new());

    Ok(router.into())
}

// fn routes_connected() -> Router {
//     todo!()
// }
