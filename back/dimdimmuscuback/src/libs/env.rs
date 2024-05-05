use std::env;

use dotenv::dotenv;
use libsql::Connection;
use rand::{rngs::OsRng, RngCore};

use crate::libs::db::methods::init_db::init_db;
use crate::libs::secret::Secret;

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

fn get_variables_from_env() -> (i64, Secret<String>, Secret<String>) {
    // Load the .env file
    dotenv().ok();

    // should not be possible to print those values anywhere
    let db_url = Secret(env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set"));
    let db_auth_token = Secret(env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set"));
    let session_duration_hours: i64 = env::var("SESSION_DURATION_HOURS")
        .expect("SESSION_DURATION_HOURS must be set")
        .parse()
        .expect("SESSION_DURATION_HOURS must be i64");

    (session_duration_hours, db_url, db_auth_token)
}

fn generate_secret_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_env_var() {
        let _ = get_variables_from_env();
    }

    #[tokio::test]
    async fn key_gen_generation() {
        // should be different each time
        assert_ne!(generate_secret_key(), generate_secret_key());
    }

    #[tokio::test]
    async fn test_db() {
        let env = init_env().await;

        // do nothing but test if connection ok
        env.db_connection.execute("SELECT 2", ()).await.unwrap();
    }
}
