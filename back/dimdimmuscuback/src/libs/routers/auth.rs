use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log;
use log::info;

use crate::libs::db::structs::session::{SessionLogoff, SessionTokenValue};
use crate::libs::db::structs::users_auth::{UserForCreate, UserForDelete, UserForLogin};
use crate::libs::env::EnvVariables;
use crate::libs::routers::fallback::fallback;

pub(super) fn auth_routes(env_variables: EnvVariables) -> Router {
    Router::new()
        .route("/signup", post(api_signup_handler))
        .route("/login", post(api_login_handler))
        .route("/logoff", post(api_logoff_handler))
        .route("/delete_user", post(api_delete_user_handler))
        .with_state(env_variables)
        .fallback(fallback)
}

async fn api_signup_handler(
    State(env_variables): State<EnvVariables>,
    Json(user): Json<UserForCreate>,
) -> impl IntoResponse {
    info!("Trying user creation for {}", &user.username);
    match user.add_new_user_in_db(&env_variables.db_connection).await {
        Ok(id) => {
            info!("User {} created", &user.username);
            get_token(&env_variables, id, StatusCode::CREATED)
                .await
                .into_response()
        }
        Err(err) => err.into_response(),
    }
}

async fn api_login_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_login): Json<UserForLogin>,
) -> impl IntoResponse {
    info!("Trying user connection for {}", &user_for_login.username);
    let profile_id = match user_for_login
        .authenticate(&env_variables.db_connection)
        .await
    {
        Ok(id) => id,
        Err(auth_error) => return auth_error.into_response(),
    };

    info!("User {} connected", &user_for_login.username);
    get_token(&env_variables, profile_id, StatusCode::OK)
        .await
        .into_response()
}

async fn get_token(
    env_variables: &EnvVariables,
    profile_id: String,
    code: StatusCode,
) -> impl IntoResponse {
    match SessionTokenValue::create(profile_id, &env_variables).await {
        Ok(token) => (code, token.get()).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn api_logoff_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_logoff): Json<SessionLogoff>,
) -> impl IntoResponse {
    info!(
        "Trying user de-connection for {}",
        user_for_logoff.get_associated_user(&env_variables)
    );
    match user_for_logoff
        .clear_session(env_variables.db_connection)
        .await
    {
        Ok(profile_name) => {
            let txt = format!("User {} logged off successfully", profile_name);
            info!("{}", txt);
            (StatusCode::OK, txt).into_response()
        }
        Err(e) => e.into_response(),
    }
}

async fn api_delete_user_handler(
    State(env_variables): State<EnvVariables>,
    Json(user_for_delete): Json<UserForDelete>,
) -> impl IntoResponse {
    info!(
        "Trying user deletion for {}",
        user_for_delete.get_associated_user(&env_variables)
    );
    match user_for_delete
        .delete_user(env_variables.db_connection)
        .await
    {
        Ok(profile_name) => {
            let txt = format!("User {} deleted successfully", profile_name);
            info!("{}", txt);
            (StatusCode::OK, txt).into_response()
        }
        Err(error) => error.into_response(),
    }
}
