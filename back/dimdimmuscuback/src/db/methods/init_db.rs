use std::process::Command;

use shuttle_runtime::CustomError;
use sqlx::postgres::PgPool;
use tokio::sync::OnceCell;

pub async fn init_db_shuttle(pool: PgPool) -> DbInfos {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)
        .expect("cannot init database");

    DbInfos { pool }
}

#[derive(Clone)]
pub struct DbInfos {
    pub pool: PgPool,
}

pub async fn init_test() -> DbInfos {
    static INIT: OnceCell<DbInfos> = OnceCell::const_new();

    INIT.get_or_init(|| async { prepare_db().await })
        .await
        .clone()
}

async fn prepare_db() -> DbInfos {
    launch_docker_db();
    let db = setup_db().await;
    populate_db(&db).await;

    db
}

fn launch_docker_db() {
    let e = Command::new("sh")
        .arg("../../scripts/db_test_no_shuttle.sh")
        .output()
        .expect("failed to execute process");
    if e.stdout.is_empty() || !e.stderr.is_empty() {
        panic!("Failed to create db");
    }
}

async fn setup_db() -> DbInfos {
    let pool = PgPool::connect("postgresql://dim:dim@localhost:3269/db_for_tests_without_shuttle")
        .await
        .expect("Failed to connect to the database for testing");

    DbInfos { pool }
}

async fn populate_db(db: &DbInfos) {}
