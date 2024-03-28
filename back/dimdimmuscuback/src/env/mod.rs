use std::env;

use libsql::Connection;
use rand::{rngs::OsRng, RngCore};

use crate::db::methods::init_db::init_db;

#[derive(Clone)]
pub struct EnvVariables {
    pub session_duration_hours: i64,
    pub secret_key_session: [u8; 32],
    pub db_connection: Connection,
}

// Should fail directly in case of issue
pub async fn init_env() -> EnvVariables {
    // from .env
    let (session_duration_hours, db_url, db_auth_token) = get_variables_from_env();

    // need to change at each restart
    let secret_key_session = generate_secret_key();

    // turso connection
    let db_connection = init_db(db_url, db_auth_token).await;

    EnvVariables {
        session_duration_hours,
        secret_key_session,
        db_connection,
    }
}

fn get_variables_from_env() -> (i64, String, String) {
    let session_duration_hours: i64 = env::var("session_duration_hours")
        .expect("")
        .parse()
        .expect("");

    let db_url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let db_auth_token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();
    (session_duration_hours, db_url, db_auth_token)
}

fn generate_secret_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    key
}
