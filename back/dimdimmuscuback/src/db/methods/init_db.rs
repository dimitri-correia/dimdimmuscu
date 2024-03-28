use libsql::Connection;
use tokio::sync::OnceCell;

pub async fn init_db(db_url: String, db_auth_token: String) -> Connection {
    static INIT: OnceCell<Connection> = OnceCell::const_new();

    INIT.get_or_init(|| async { prepare_db(db_url, db_auth_token).await })
        .await
        .clone()
}

async fn prepare_db(db_url: String, db_auth_token: String) -> Connection {
    use libsql::Builder;

    let db = Builder::new_remote(db_url, db_auth_token)
        .build()
        .await
        .unwrap();

    db.connect().unwrap()
}
