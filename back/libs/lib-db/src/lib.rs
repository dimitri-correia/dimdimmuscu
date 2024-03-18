use sqlx::PgPool;

#[derive(Clone)]
pub struct DbInfos {
    pub pool: PgPool,
}

#[cfg(test)]
pub mod tests {
    use std::process::Command;
    use std::sync::OnceLock;

    use super::*;

    pub async fn init_test() -> DbInfos {
        static INIT: OnceLock<DbInfos> = OnceLock::new();

        let db = INIT.get_or_init(|| async { prepare_db().await });

        db.clone()
    }

    async fn prepare_db() -> DbInfos {
        launch_docker_db();
        let db = setup_db().await;
        populate_db(&db).await;

        db
    }

    fn launch_docker_db() {
        Command::new("sh")
            .arg("scripts/db_test_no_shuttle.sh")
            .output()
            .expect("failed to execute process");
    }

    async fn setup_db() -> DbInfos {
        let pool =
            PgPool::connect("postgresql://dim:dim@localhost:3269/db_for_tests_without_shuttle")
                .await
                .expect("Failed to connect to the database for testing");

        DbInfos { pool }
    }

    async fn populate_db(db: &DbInfos) {}
}
