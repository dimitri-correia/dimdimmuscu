use axum::Router;
use sqlx::PgPool;

use crate::db::methods::init_db::{init_db, DbInfos};
use crate::routers::auth::auth_routes;
use crate::routers::fallback::fallback;

mod db;
mod env;
mod errors;
mod logic;
mod mw;
mod routers;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    init_db(&pool).await;

    let db = DbInfos { pool };

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
