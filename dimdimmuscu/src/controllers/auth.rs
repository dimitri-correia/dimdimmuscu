use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    _entities::users,
    users::{LoginParams, RegisterParams},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

/// Register function creates a new user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    dbg!(&user);

    format::json(user)
}

#[derive(Serialize)]
struct LoginResponse {
    user: users::Model,
    token: String,
}

impl LoginResponse {
    fn new(user: &users::Model, token: &str) -> Self {
        Self {
            user: user.clone(),
            token: token.to_string(),
        }
    }
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    let valid = user.verify_password(&params.password);

    if !valid {
        return unauthorized("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth")
        .add("/login", post(login))
        .add("/register", post(register))
}
