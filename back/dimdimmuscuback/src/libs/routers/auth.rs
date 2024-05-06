use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log;
use log::info;

use crate::libs::db::structs::session::{SessionLogoff, SessionTokenValue};
use crate::libs::db::structs::users_auth::{UserForCreate, UserForDelete, UserForLogin};
use crate::libs::env::EnvVariables;
use crate::libs::routers::fallback::fallback;

pub fn auth_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/signup", post(api_signup_handler))
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
        .route("/delete_user", post(api_delete_user_handler))
        .with_state(env_variables)
        .fallback(fallback)
}

async fn api_signup_handler(
    State(env_variables): State<EnvVariables>,
    Json(user): Json<UserForCreate>,
) -> impl IntoResponse {
    info!("Trying signup for {}", user.username);
    if let Some(creation) = user
        .add_new_user_in_db(&env_variables.db_connection)
        .await
        .err()
    {
        return creation.into_response();
    };

    // we choose to not give a cookie here but to force authentication with /login endpoint
    (StatusCode::CREATED, "User creation worked".to_string()).into_response()
}

async fn api_login_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_login): Json<UserForLogin>,
) -> impl IntoResponse {
    let profile_id = match user_for_login
        .authenticate(&env_variables.db_connection)
        .await
    {
        Ok(id) => id,
        Err(auth_error) => return auth_error.into_response(),
    };

    match SessionTokenValue::create(profile_id, &env_variables).await {
        Ok(token) => (StatusCode::OK, token.get()).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn api_logoff_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_logoff): Json<SessionLogoff>,
) -> impl IntoResponse {
    match user_for_logoff
        .clear_session(env_variables.db_connection)
        .await
    {
        Ok(profile_name) => (
            StatusCode::OK,
            format!("User {} logged off successfully", profile_name).to_string(),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}

async fn api_delete_user_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_delete): Json<UserForDelete>,
) -> impl IntoResponse {
    match user_for_delete
        .delete_user(env_variables.db_connection)
        .await
    {
        Ok(profile_name) => (
            StatusCode::OK,
            format!("User {} deleted successfully", profile_name).to_string(),
        )
            .into_response(),
        Err(error) => error.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestResponse, TestServer};
    use chrono::Utc;
    use rand::Rng;
    use serde_json::json;

    use crate::libs::env::init_env;
    use crate::test_helper::tests_helper::get_secret_store_for_tests;

    use super::*;

    #[tokio::test]
    async fn test_end_to_end_auth() {
        let env = init_env(get_secret_store_for_tests()).await;
        let app = auth_routes(env);

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let username = format!(
            "test_user_for_test_end_to_end_auth_{}",
            rand::rngs::OsRng.gen::<u32>()
        );
        let pwd_clear = "pwd_clear";

        // User creation
        {
            let response = create_user(&server, &username, pwd_clear).await;
            response.assert_status(StatusCode::CREATED);
        }

        // Can't have the same username
        {
            let response = create_user(&server, &username, pwd_clear).await;
            response.assert_status(StatusCode::UNAUTHORIZED);
        }

        // User auth
        let token: String;
        {
            let response = user_login(&server, &username, pwd_clear).await;
            response.assert_status(StatusCode::OK);
            token = response.text();
        }

        // User logoff
        {
            let response = server.post("/logoff").json(&json!({"token": token,})).await;
            response.assert_status(StatusCode::OK);
            dbg!(response.text());
            response.assert_text(format!("User {} logged off successfully", username));
        }

        // Auth again
        let token: String;
        {
            let response = user_login(&server, &username, pwd_clear).await;
            response.assert_status(StatusCode::OK);
            token = response.text();
        }

        // User delete
        {
            let response = server
                .post("/delete_user")
                .json(&json!({"token": token,}))
                .await;

            response.assert_status(StatusCode::OK);
            response.assert_text(format!("User {} deleted successfully", username));
        }

        // Cant log anymore
        {
            let response = user_login(&server, &username, pwd_clear).await;
            response.assert_status(StatusCode::UNAUTHORIZED);
        }
    }

    async fn user_login(server: &TestServer, username: &String, pwd_clear: &str) -> TestResponse {
        server
            .post("/login")
            .json(&json!({
                "username": username,
                "pwd_clear": pwd_clear,
            }
            ))
            .await
    }

    async fn create_user(server: &TestServer, username: &String, pwd_clear: &str) -> TestResponse {
        server
            .post("/signup")
            .json(&json!({
                "username": username,
                "pwd_clear": pwd_clear,
                "birthdate": Utc::now().to_rfc3339(),
            }
            ))
            .await
    }
}
