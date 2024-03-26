use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct DbInfos {
    pub pool: PgPool,
}

pub async fn init_db(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Cannot init database");
}

#[cfg(test)]
pub mod base_tests_db {
    use std::thread::sleep;
    use std::time;
    use std::time::Duration;

    use sqlx::{Error, PgPool};
    use testcontainers::clients::Cli;
    use testcontainers_modules::postgres::Postgres;
    use tokio::sync::OnceCell;

    use crate::db::methods::init_db::{init_db, DbInfos};

    pub async fn init_test() -> DbInfos {
        static INIT: OnceCell<DbInfos> = OnceCell::const_new();

        INIT.get_or_init(|| async { prepare_db().await })
            .await
            .clone()
    }

    pub async fn check_db_health(pool: &PgPool) -> Result<(), Error> {
        sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
    }

    async fn prepare_db() -> DbInfos {
        let db = setup_db().await;
        dbg!("a");
        sleep(time::Duration::from_secs_f32(3f32));
        dbg!("b");
        for _ in 0..5 {
            if let Ok(_) = check_db_health(&db.pool).await {
                break;
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        init_db(&db.pool).await;

        // Check the database health
        check_db_health(&db.pool)
            .await
            .expect("Database health check failed");

        db
    }

    async fn setup_db() -> DbInfos {
        let docker = Cli::default();
        let node = docker.run(Postgres::default());

        // prepare connection string
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );

        let pool: PgPool = PgPool::connect(connection_string).await.unwrap();

        DbInfos { pool }
    }

    async fn populate_db(pool: &PgPool) {
        dbg!("c");
    }
}
