use axum::http::StatusCode;
use axum_test::{TestResponse, TestServer};
use chrono::Utc;
use rand::Rng;
use serde_json::json;

use dimdimmuscuback::libs::env::init_env;
use dimdimmuscuback::libs::routers::auth::auth_routes;

use crate::test_helper::get_secret_store_for_tests;

pub mod test_helper;

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