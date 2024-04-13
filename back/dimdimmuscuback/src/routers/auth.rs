use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use crate::db::structs::session::{Session, SessionLogoff, SessionToken};
use crate::db::structs::users_auth::{UserForCreate, UserForDelete, UserForLogin};
use crate::env::EnvVariables;

pub fn auth_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/signup", post(api_signup_handler))
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
        .route("/delete_user", post(api_delete_user_handler))
        .with_state(env_variables)
}

async fn api_signup_handler(
    State(env_variables): State<EnvVariables>,
    Json(user): Json<UserForCreate>,
) -> impl IntoResponse {
    if let Some(creation) = user
        .add_new_user_in_db(&env_variables.db_connection)
        .await
        .err()
    {
        return creation.error_to_show();
    };

    // we choose to not give a cookie here but to force authentication with /login endpoint
    (StatusCode::CREATED, "User creation worked".to_string())
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
        Err(auth_error) => return auth_error.error_to_show(),
    };

    match SessionToken::create(profile_id, &env_variables).await {
        Ok(token) => (StatusCode::OK, token),
        Err(err) => err.error_to_show(),
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
        ),
        Err(e) => e.error_to_show(),
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
        ),
        Err(error) => error.error_to_show(),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestResponse, TestServer};
    use chrono::Utc;
    use rand::Rng;
    use serde_json::json;

    use crate::env::init_env;

    use super::*;

    #[tokio::test]
    async fn test_end_to_end_auth() {
        let env = init_env().await;
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
