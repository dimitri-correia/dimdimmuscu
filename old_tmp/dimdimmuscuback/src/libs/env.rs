use libsql::Connection;
use rand::{rngs::OsRng, RngCore};
use redact::Secret;
use shuttle_runtime::SecretStore;

use crate::libs::db::methods::init_db::init_db;

#[derive(Clone)]
pub struct EnvVariables {
    pub session_duration_hours: i64,
    pub secret_key_session: Secret<[u8; 32]>,
    pub db_connection: Connection,
}

// Should fail directly in case of issue
pub async fn init_env(secrets: SecretStore) -> EnvVariables {
    let (session_duration_hours, db_url, db_auth_token) = get_variables_from_secret(secrets);

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

fn get_variables_from_secret(secrets: SecretStore) -> (i64, Secret<String>, Secret<String>) {
    let db_url = Secret::new(
        secrets
            .get("TURSO_DATABASE_URL")
            .expect("TURSO_DATABASE_URL must be set"),
    );
    let db_auth_token = Secret::new(
        secrets
            .get("TURSO_AUTH_TOKEN")
            .expect("TURSO_AUTH_TOKEN must be set"),
    );
    let session_duration_hours = secrets
        .get("SESSION_DURATION_HOURS")
        .expect("SESSION_DURATION_HOURS must be set")
        .parse()
        .expect("SESSION_DURATION_HOURS must be i64");

    (session_duration_hours, db_url, db_auth_token)
}

pub fn generate_secret_key() -> Secret<[u8; 32]> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    Secret::new(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn key_gen_generation() {
        // should be different each time
        assert_ne!(generate_secret_key(), generate_secret_key());
    }
}
