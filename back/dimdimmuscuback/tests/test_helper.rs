use std::fmt::Error;
use std::fs;

use axum::Router;
use axum_test::{TestResponse, TestServer};
use chrono::Utc;
use rand::Rng;
use redact::Secret;
use serde_json::json;
use shuttle_runtime::SecretStore;
use toml::Value;

use dimdimmuscuback::libs::db::structs::users_auth::UserForCreate;
use dimdimmuscuback::libs::env::{EnvVariables, init_env};

pub fn get_secret_store_for_tests() -> SecretStore {
    let mut secrets = std::collections::BTreeMap::new();

    let db_url = get_env_or_toml("TURSO_DATABASE_URL");
    secrets.insert(
        "TURSO_DATABASE_URL".to_string(),
        shuttle_common::secrets::Secret::new(db_url),
    );

    let auth_token = get_env_or_toml("TURSO_AUTH_TOKEN");
    secrets.insert(
        "TURSO_AUTH_TOKEN".to_string(),
        shuttle_common::secrets::Secret::new(auth_token),
    );

    let session_duration_hours = get_env_or_toml("SESSION_DURATION_HOURS");
    secrets.insert(
        "SESSION_DURATION_HOURS".to_string(),
        shuttle_common::secrets::Secret::new(session_duration_hours),
    );

    SecretStore::new(secrets)
}

fn get_env_or_toml(var_name: &str) -> String {
    match std::env::var(var_name) {
        Ok(val) => val,
        Err(_) => {
            // todo set to relative path
            let contents =
                fs::read_to_string("/home/dim/RustroverProjects/dimdimmuscu/Secrets.dev.toml")
                    .expect("Something went wrong reading the file");
            let value: Value = toml::from_str(&contents).unwrap();
            value[var_name].as_str().unwrap().to_string()
        }
    }
}

pub async fn create_user(
    server: &TestServer,
    username: Option<&str>,
    pwd_clear: Option<&str>,
    height_cm: Option<u8>,
) -> TestResponse {
    let default_username = format!(
        "test_user_for_test_end_to_end_auth_{}",
        rand::rngs::OsRng.gen::<u32>()
    );
    let username = username
        .unwrap_or(&default_username);

    let pwd_clear = pwd_clear.unwrap_or(&"pwd_clear");

    server
        .post("/connect/signup")
        .json(&json!({
            "username": username,
            "pwd_clear": pwd_clear,
            "birthdate": Utc::now().to_rfc3339(),
            "height_cm": height_cm.unwrap_or(180),
        }
        ))
        .await
}

pub async fn create_user_and_get_token(
    test_server: &TestServer,
    username: Option<&str>,
    pwd_clear: Option<&str>,
) -> String {
    create_user(test_server, username, pwd_clear).await.text()
}
