use shuttle_runtime::CustomError;
use sqlx::PgPool;

use crate::db::structs::db_infos::DbInfos;

pub async fn init_db(pool: PgPool) -> DbInfos {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)
        .expect("cannot init database");

    DbInfos { pool }
}
