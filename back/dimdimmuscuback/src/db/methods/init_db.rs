use shuttle_runtime::CustomError;
use sqlx::PgPool;

use lib_db::DbInfos;

pub async fn init_db(pool: PgPool) -> DbInfos {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)
        .expect("cannot init database");

    DbInfos { pool }
}
