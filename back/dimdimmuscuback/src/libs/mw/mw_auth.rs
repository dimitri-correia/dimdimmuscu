use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};

use crate::libs::env::EnvVariables;
use crate::libs::errors::auth_errors::session_errors::SessionError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub profile_id: String,
    pub until: DateTime<Utc>,
}

#[async_trait]
impl FromRequestParts<EnvVariables> for SessionToken {
    type Rejection = SessionError;

    async fn from_request_parts(
        parts: &mut Parts,
        env_variables: &EnvVariables,
    ) -> Result<Self, Self::Rejection> {
        info!("trying to connect");
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| SessionError::BadToken)?;

        let token_data = decode::<SessionToken>(
            bearer.token(),
            &DecodingKey::from_secret(env_variables.secret_key_session.expose_secret()),
            &Validation::default(),
        )
        .map_err(|_| SessionError::BadToken)?;

        Ok(token_data.claims)
    }
}

pub async fn mw_auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if token_is_valid(&headers) {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn token_is_valid(header_map: &HeaderMap) -> bool {
    header_map
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    true
}
