use std::env;

use dotenv::dotenv;
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
    // Load the .env file
    dotenv().ok();

    let session_duration_hours: i64 = env::var("SESSION_DURATION_HOUR")
        .expect("SESSION_DURATION_HOUR must be set")
        .parse()
        .expect("SESSION_DURATION_HOUR must be i64");

    let db_url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let db_auth_token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();
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

        env.db_connection
            .execute_batch(
                r#"-- sqlite

PRAGMA foreign_keys = ON;

CREATE TABLE users
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    name             VARCHAR(256) NOT NULL,
    birthdate        TEXT         NOT NULL,
    account_creation TEXT         NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);

-- Create an index to have faster queries on name
CREATE INDEX idx_users_name ON users (name);

CREATE TABLE users_auth
(
    profile_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE PRIMARY KEY,
    pwd        VARCHAR(256),
    CONSTRAINT unique_profile_id UNIQUE (profile_id)
);

CREATE TABLE session
(
    token      VARCHAR(256) NOT NULL PRIMARY KEY,
    profile_id INTEGER      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    until      TEXT         NOT NULL
);

CREATE TABLE muscles
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(256) NOT NULL,
    CONSTRAINT unique_muscle_name UNIQUE (name)
);

CREATE TABLE movements
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          VARCHAR(256) NOT NULL,
    first_muscle  INTEGER      NOT NULL REFERENCES muscles (id),
    second_muscle INTEGER REFERENCES muscles (id),
    third_muscle  INTEGER REFERENCES muscles (id),
    CONSTRAINT unique_movement_name UNIQUE (name)
);

CREATE TABLE lifts
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    lift_time   TEXT
);

CREATE TABLE sets
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    lift_id    INTEGER NOT NULL REFERENCES lifts (id) ON DELETE CASCADE,
    set_number INTEGER NOT NULL,
    number_rep INTEGER NOT NULL,
    weight     INTEGER NOT NULL,
    CONSTRAINT unique_set_number_per_lift UNIQUE (set_number, lift_id)
);

CREATE TABLE maxes
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    number_rep  INTEGER NOT NULL,
    weight      INTEGER NOT NULL,
    CONSTRAINT unique_max_per_profile_movement UNIQUE (profile_id, movement_id)
);
"#,
            )
            .await
            .unwrap();
    }
}
