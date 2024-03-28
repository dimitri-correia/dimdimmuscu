use std::env;

use rand::{rngs::OsRng, RngCore};

pub struct EnvVariables {
    session_duration_hours: i64,
    secret_key_session: [u8; 32],
}

pub struct EnvVariablesDb {
    pub db_url: String,
    pub db_auth_token: String,
}

// Should fail directly in case of issue
pub fn init_env() -> (EnvVariables, EnvVariablesDb) {
    let session_duration_hours: i64 = env::var("session_duration_hours")
        .expect("")
        .parse()
        .expect("");

    let db_url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let db_auth_token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let secret_key_session = generate_secret_key(); // need to change at each restart

    (
        EnvVariables {
            session_duration_hours,
            secret_key_session,
        },
        EnvVariablesDb {
            db_url,
            db_auth_token,
        },
    )
}

fn generate_secret_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    key
}
