use axum::Router;
use shuttle_runtime::__internals::tracing_subscriber;
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use crate::db::methods::init_db::init_db_shuttle;
use crate::routers::fallback::fallback;

mod cookies;
mod db;
mod errors;
mod logic;
mod routers;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .fallback(fallback)
        .layer(CookieManagerLayer::new())
        .with_state(init_db_shuttle(pool).await);

    info!("started");

    Ok(router.into())
}
