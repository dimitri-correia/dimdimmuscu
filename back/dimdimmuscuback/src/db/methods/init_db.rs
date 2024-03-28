use libsql::Connection;
use tokio::sync::OnceCell;

use crate::env::EnvVariablesDb;

#[derive(Clone)]
pub struct DbInfos {
    pub conn: Connection,
}

pub async fn init_db(env_variables_db: EnvVariablesDb) -> DbInfos {
    static INIT: OnceCell<DbInfos> = OnceCell::const_new();

    INIT.get_or_init(|| async { prepare_db(env_variables_db).await })
        .await
        .clone()
}

async fn prepare_db(env_variables_db: EnvVariablesDb) -> DbInfos {
    use libsql::Builder;

    let db = Builder::new_remote(env_variables_db.db_url, env_variables_db.db_auth_token)
        .build()
        .await
        .unwrap();
    let conn = db.connect().unwrap();

    DbInfos { conn }
}
