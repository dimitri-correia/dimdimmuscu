use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

use lib_auth::pwd::{self, ContentToHash, SchemeStatus};

use crate::cookies;
use crate::db::methods::init_db::DbInfos;
use crate::db::structs::users::UserForLogin;
use crate::errors::login::LoginError;

pub fn login_routes(db_infos: DbInfos) -> Router {
    Router::new()
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
        .with_state(db_infos)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login_handler(
    State(db_infos): State<DbInfos>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    debug!("attempt to login");

    let user: UserForLogin = UserForLogin::find_by_name(&db_infos.pool, &payload.username)
        .await
        .map_err(|_| LoginError::LoginFailUsernameNotFound.login_failed())?;

    let scheme_status = pwd::validate_pwd(
        ContentToHash {
            salt: user.pwd_salt,
            content: payload.pwd.clone(),
        },
        user.pwd,
    )
    .await
    .map_err(|_| LoginError::LoginFailPwdNotMatching { user_id: user.id }.login_failed())?;

    // -- Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading.");
        UserForLogin::update_pwd(&db_infos.pool, &payload.username, &payload.pwd)
            .await
            .map_err(|_| LoginError::FailUpdate.login_failed())?;
    }

    cookies::set_token_cookie(&cookies, &payload.username, user.token_salt)
        .map_err(|_| LoginError::SetCookieFailed.login_failed())?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

async fn api_logoff_handler(cookies: Cookies) -> Json<Value> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");

    cookies::remove_token_cookie(&cookies);

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
    use crate::db::methods::init_db::init_test;

    use super::*;

    #[tokio::test]
    async fn task1() {
        let db = init_test().await;
        let app = login_routes(db);

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
