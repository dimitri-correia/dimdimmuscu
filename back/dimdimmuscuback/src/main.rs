use axum::Router;
use sqlx::PgPool;

use crate::db::methods::init_db::init_db;
use crate::routers::fallback::fallback;

mod db;
mod logic;
mod routers;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .fallback(fallback)
        .with_state(init_db(pool).await);

    Ok(router.into())
}
