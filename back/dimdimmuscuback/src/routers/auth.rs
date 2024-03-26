use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{http, Json, Router};
use http_body::Empty;
use serde_json::{json, Value};
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

use crate::db::methods::init_db::DbInfos;
use crate::db::structs::session::Session;
use crate::db::structs::users_auth::{UserForCreate, UserForLogin};
use crate::env::{COOKIE_MAX_AGE, COOKIE_MAX_AGE_DAY, USER_COOKIE_NAME};

pub fn auth_routes(db_infos: DbInfos) -> Router {
    Router::new()
        .route("/signup", post(api_signup_handler))
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
        .with_state(db_infos)
}

async fn api_signup_handler(
    State(db_infos): State<DbInfos>,
    Json(user): Json<UserForCreate>,
) -> impl IntoResponse {
    if let Some(creation) = user.add_new_user_in_db(&db_infos.pool).await.err() {
        return creation.error_to_show();
    };

    // we choose to not give a cookie here but to force authentication with /login endpoint
    (StatusCode::CREATED, "User creation worked".to_string())
}

async fn api_login_handler(
    State(db_infos): State<DbInfos>,
    cookies: Cookies,
    Json(user_for_login): Json<UserForLogin>,
) -> impl IntoResponse {
    let profile_id = match user_for_login.authenticate(&db_infos.pool).await {
        Ok(id) => id,
        Err(auth_error) => return auth_error.error_to_show(),
    };

    let token = match Session::new(profile_id, &db_infos.pool).await {
        Ok(t) => t,
        Err(err) => return err.error_to_show(),
    };

    let cookie: Cookie = Cookie::build((USER_COOKIE_NAME, token.clone()))
        .domain("dimdimmuscu")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(COOKIE_MAX_AGE_DAY))
        .build();

    cookies.add(cookie);

    (StatusCode::CREATED, format!("token: {}", token)) // todo
}

async fn api_logoff_handler(
    State(db_infos): State<DbInfos>,
    cookies: Cookies,
) -> impl IntoResponse {
    cookies.remove(cookies);

    Json(json!({
        "result": {
            "logged_off": true
        }
    }))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    use crate::cookies::AUTH_TOKEN;
    use crate::db::methods::init_db::base_tests_db::init_test;

    use super::*;

    #[tokio::test]
    async fn signup_ok_test() {
        let db = init_test().await;
        let app = auth_routes(db);

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/signup")
            .json(&json!({
                "username": "String",
                "pwd_clear": "String",
                "birthdate": "String"
            }
            ))
            .await;

        response.assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn login_ok_test() {
        let db = init_test().await;
        let app = auth_routes(db);

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.post("/login").await;

        response.assert_status(StatusCode::OK);

        assert!(response.cookies().get(AUTH_TOKEN).is_some());

        response.assert_json(&json!({
        "result": {
            "success": true
        }
        }));
    }
}
