use axum::extract::State;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use redact::Secret;
use serde::{Deserialize, Serialize};

use crate::libs::db::structs::session::SessionTokenValue;
use crate::libs::env::EnvVariables;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub profile_id: String,
    pub until: DateTime<Utc>,
}

impl SessionToken {
    pub fn encode(
        &self,
        secret_key_session: &Secret<[u8; 32]>,
    ) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret_key_session.expose_secret()),
        )
    }
}

pub async fn mw_auth(
    State(env_variables): State<EnvVariables>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    info!("trying to connect");
    if let Ok(profile_id) =
        SessionTokenValue::validate_token(auth.token().to_string(), &env_variables).await
    {
        request.extensions_mut().insert(profile_id);
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
