use axum::http::StatusCode;
use axum_test::{TestResponse, TestServer};
use rand::Rng;
use serde_json::json;

use dimdimmuscuback::libs::routers::main_router::main_router;

use crate::test_helper::{create_user, get_secret_store_for_tests};

pub mod test_helper;

#[tokio::test]
async fn test_end_to_end_auth() {
    let app = main_router(get_secret_store_for_tests()).await;

    // Run the application for testing.
    let server = TestServer::new(app).unwrap();

    let username = format!(
        "test_user_for_test_end_to_end_auth_{}",
        rand::rngs::OsRng.gen::<u32>()
    );
    let pwd_clear = "pwd_clear";

    // User creation
    let token_from_creation: String;
    {
        let response = create_user(&server, Some(&username), Some(pwd_clear), Some(180)).await;
        response.assert_status(StatusCode::CREATED);
        token_from_creation = response.text();
    }

    // Can't have the same username
    {
        let response = create_user(&server, Some(&username), Some(pwd_clear), Some(180)).await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }

    // User auth
    let token: String;
    {
        let response = user_login(&server, &username, pwd_clear).await;
        response.assert_status(StatusCode::OK);
        token = response.text();
        assert_ne!(token, token_from_creation);
    }

    // User logoff
    {
        let response = server
            .post("/connect/logoff")
            .json(&json!({"token": token,}))
            .await;
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
            .post("/connect/delete_user")
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
        .post("/connect/login")
        .json(&json!({
            "username": username,
            "pwd_clear": pwd_clear,
        }
        ))
        .await
}
