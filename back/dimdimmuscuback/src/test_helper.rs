#[cfg(test)]
pub mod tests_helper {
    use std::fmt::Error;
    use std::fs;

    use chrono::Utc;
    use rand::Rng;
    use redact::Secret;
    use shuttle_runtime::SecretStore;
    use toml::Value;

    use crate::libs::db::structs::session::SessionTokenValue;
    use crate::libs::db::structs::users_auth::{UserForCreate, UserForLogin};
    use crate::libs::env::{init_env, EnvVariables};

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
                    fs::read_to_string("/home/dim/RustroverProjects/DimDimMuscu/Secrets.dev.toml")
                        .expect("Something went wrong reading the file");
                let value: Value = toml::from_str(&contents).unwrap();
                value[var_name].as_str().unwrap().to_string()
            }
        }
    }

    pub async fn create_user_test_helper() -> (EnvVariables, SessionTokenValue) {
        let env = init_env(get_secret_store_for_tests()).await;

        let username = format!(
            "test_user_for_test_end_to_end_auth_{}",
            rand::rngs::OsRng.gen::<u32>()
        );
        let pwd_clear = "pwd_clear";

        let user = UserForCreate::_create(
            username.clone(),
            Secret::new(pwd_clear.to_string()),
            Utc::now().to_rfc3339(),
        );
        user.add_new_user_in_db(&env.db_connection)
            .await
            .map_err(|_| Error)
            .expect("should add without issue");

        let user_for_login = UserForLogin::_create(username, pwd_clear.to_string());
        let profile_id = user_for_login
            .authenticate(&env.db_connection)
            .await
            .map_err(|_| Error)
            .expect("should connect without issue");

        let token = SessionTokenValue::create(profile_id, &env)
            .await
            .map_err(|_| Error)
            .expect("should give back the token");

        (env, token)
    }
}
