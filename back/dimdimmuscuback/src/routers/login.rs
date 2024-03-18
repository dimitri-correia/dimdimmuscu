use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

use lib_auth::pwd::{self, ContentToHash, SchemeStatus};

use crate::db::structs::auth_users::UserForLogin;
use crate::db::structs::db_infos::DbInfos;
use crate::errors::login::LoginError;

pub fn routes(db_infos: DbInfos) -> Router {
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
) -> Result<Json<Value>, LoginError> {
    debug!("attempt to login");

    let user: UserForLogin = UserForLogin::find_by_name(&db_infos.pool, &payload.username)
        .await?
        .ok_or(LoginError::LoginFailUsernameNotFound)?;

    let scheme_status = pwd::validate_pwd(
        ContentToHash {
            salt: user.pwd_salt,
            content: payload.pwd.clone(),
        },
        user.pwd,
    )
    .await
    .map_err(|_| LoginError::LoginFailPwdNotMatching { user_id })?;

    // -- Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading.");
        // todo update pwd
    }

    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}

async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}
