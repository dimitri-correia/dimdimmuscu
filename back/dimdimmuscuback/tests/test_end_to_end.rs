use axum::http::HeaderValue;
use axum_test::http::HeaderName;
use axum_test::TestServer;

use dimdimmuscuback::libs::routers::main_router::main_router;

use crate::test_helper::{create_user_test_helper, get_secret_store_for_tests};

pub mod test_helper;

#[tokio::test]
async fn test_end_to_end_normal_use() {
    let app = main_router(get_secret_store_for_tests()).await;

    // Run the application for testing.
    let server = TestServer::new(app).unwrap();

    let (env_variables, token) = create_user_test_helper().await;

    // Get myself
    {
        let response = server
            .get("/api/get_myself")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_str(&token).unwrap(),
            )
            .await;
        dbg!(response);
    }
}
