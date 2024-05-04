use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};

use crate::libs::env::EnvVariables;
use crate::libs::errors::auth::session::SessionError;

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
            &DecodingKey::from_secret(&env_variables.secret_key_session),
            &Validation::default(),
        )
        .map_err(|_| SessionError::BadToken)?;

        Ok(token_data.claims)
    }
}
