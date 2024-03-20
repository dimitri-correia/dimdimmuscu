use axum::{middleware, Router};
use shuttle_runtime::__internals::tracing_subscriber;
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use crate::db::methods::init_db::init_db_shuttle;
use crate::routers::fallback::fallback;
use crate::routers::login::login_routes;

mod cookies;
mod db;
mod errors;
mod logic;
mod mw;
mod routers;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt().init();

    let db = init_db_shuttle(pool).await;

    let routes_connected =
        web::routes_rpc::routes(db.clone()).route_layer(middleware::from_fn(mw_ctx_require));

    let router = Router::new()
        .nest("/connect", login_routes(db.clone()))
        .nest("/api", routes_connected())
        .fallback(fallback)
        .layer(CookieManagerLayer::new());

    info!("started");

    Ok(router.into())
}
