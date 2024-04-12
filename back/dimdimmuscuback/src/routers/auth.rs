use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use crate::db::structs::session::{Session, SessionLogoff};
use crate::db::structs::users_auth::{UserForCreate, UserForLogin};
use crate::env::EnvVariables;

pub fn auth_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/signup", post(api_signup_handler))
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
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

    let token: String = match Session::create(profile_id, &env_variables.db_connection).await {
        Ok(t) => t,
        Err(err) => return err.error_to_show(),
    };

    (StatusCode::OK, format!("token: {}", token))
}

async fn api_logoff_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_logoff): Json<SessionLogoff>,
) -> impl IntoResponse {
    user_for_logoff
        .clear_session(env_variables.db_connection)
        .await;
    (StatusCode::OK, "Logged off successfully".to_string())
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use chrono::Utc;
    use serde_json::json;

    use crate::env::init_env;

    use super::*;

    #[tokio::test]
    async fn test_end_to_end_auth() {
        let env = init_env().await;
        let app = auth_routes(env);

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let username = "test_user";
        let pwd_clear = "pwd_clear";

        // User creation
        {
            let response = server
                .post("/signup")
                .json(&json!({
                    "username": username,
                    "pwd_clear": pwd_clear,
                    "birthdate": Utc::now().to_rfc3339(),
                }
                ))
                .await;

            response.assert_status(StatusCode::CREATED);
        }

        // User auth
        {
            let response = server
                .post("/login")
                .json(&json!({
                    "username": username,
                    "pwd_clear": pwd_clear,
                }
                ))
                .await;

            response.assert_status(StatusCode::OK);
        }

        // User logoff
        {}

        // User delete
        {}
    }
}
