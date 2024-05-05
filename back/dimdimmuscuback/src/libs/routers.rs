pub mod auth;
pub mod connected;
pub mod fallback;
pub mod main_router;

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use chrono::Utc;
    use rand::Rng;

    use crate::libs::db::structs::session::SessionTokenValue;
    use crate::libs::db::structs::users_auth::{UserForCreate, UserForLogin};
    use crate::libs::env::{init_env, EnvVariables};
    use crate::libs::secret::Secret;

    pub async fn create_user_test_helper() -> (EnvVariables, SessionTokenValue) {
        let env = init_env().await;

        let username = format!(
            "test_user_for_test_end_to_end_auth_{}",
            rand::rngs::OsRng.gen::<u32>()
        );
        let pwd_clear = "pwd_clear";

        let user = UserForCreate::_create(
            username.clone(),
            Secret(pwd_clear.to_string()),
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

        return (env, token);
    }
}
