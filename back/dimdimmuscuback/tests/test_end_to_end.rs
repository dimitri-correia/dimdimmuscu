use axum::http::HeaderValue;
use axum_test::http::{HeaderName, StatusCode};
use axum_test::TestServer;

use dimdimmuscuback::libs::routers::main_router::main_router;

use crate::test_helper::{create_user_and_get_token, get_secret_store_for_tests};

pub mod test_helper;

#[tokio::test]
async fn test_end_to_end_normal_use() {
    let app = main_router(get_secret_store_for_tests()).await;

    // Run the application for testing.
    let server = TestServer::new(app).unwrap();

    let token = create_user_and_get_token(&server, None, None, None).await;
    dbg!(&token);

    // Get myself
    {
        let response = server
            .get("/api/users/get_myself")
            .add_header(
                HeaderName::from_lowercase(b"authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            )
            .await;
        dbg!(&response);
        response.assert_status(StatusCode::OK);
    }
}
