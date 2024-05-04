use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use log::info;

use crate::libs::env::EnvVariables;
use crate::libs::mw::mw_auth::SessionToken;

pub fn muscles_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/add", post(add_new))
        .with_state(env_variables)
}

async fn add_new(
    token: SessionToken,
    State(env_variables): State<EnvVariables>,
    // Json(muscle): Json<UserForCreate>,
) -> impl IntoResponse {
    info!("{} is adding {}", "token.profile_id", "muscle.username");
    (StatusCode::CREATED, "Muscle creation worked".to_string()).into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::http::header::AUTHORIZATION;
    use axum_test::http::HeaderValue;
    use axum_test::TestServer;

    use crate::libs::routers::connected::muscles::muscles_routes;
    use crate::libs::routers::tests::create_user_test_helper;

    #[tokio::test]
    async fn test_add_muscle() {
        let (env, token) = create_user_test_helper().await;
        let app = muscles_routes(env);

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/add")
            .add_header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            )
            .await;

        response.assert_status(StatusCode::OK);

        panic!();
    }
}
