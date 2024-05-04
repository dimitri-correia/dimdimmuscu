use libsql::Connection;
use tokio::sync::OnceCell;

use crate::libs::env::Secret;

pub async fn init_db(db_url: Secret<String>, db_auth_token: Secret<String>) -> Connection {
    static INIT: OnceCell<Connection> = OnceCell::const_new();

    INIT.get_or_init(|| async { prepare_db(db_url, db_auth_token).await })
        .await
        .clone()
}

async fn prepare_db(db_url: Secret<String>, db_auth_token: Secret<String>) -> Connection {
    use libsql::Builder;

    let db = Builder::new_remote(db_url.0, db_auth_token.0)
        .build()
        .await
        .unwrap();

    db.connect().unwrap()
}

// to use when creating a new db
// async fn _create_db_online() {
//     let init_db = include_str!("../../../migrations/00000_init_db.sql");
//     let env = init_env().await;
//     env.db_connection
//         .execute_batch(init_db)
//         .await
//         .expect("Create db should work");
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn create_db() {
//         _create_db_online().await;
//     }
// }
