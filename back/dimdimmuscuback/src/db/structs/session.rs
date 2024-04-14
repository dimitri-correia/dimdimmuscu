use std::fmt::Display;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use libsql::Connection;
use serde::{Deserialize, Serialize};

use crate::db::methods::queries::insert;
use crate::db::{SESSION_TABLE, SESSION_TABLE_COL};
use crate::env::EnvVariables;
use crate::errors::auth::session::SessionError;
use crate::mw::mw_auth::SessionToken;

#[derive(Deserialize, Serialize)]
pub struct SessionTokenValue(String);

impl Display for SessionTokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl SessionTokenValue {
    pub async fn create(
        profile_id: String,
        env_variables: &EnvVariables,
    ) -> Result<SessionTokenValue, SessionError> {
        // multiple session for a single user can live at the same time
        let until = Utc::now() + Duration::hours(env_variables.session_duration_hours);

        let token = encode(
            &Header::default(),
            &SessionToken {
                profile_id: profile_id.clone(),
                until,
            },
            &EncodingKey::from_secret(&env_variables.secret_key_session),
        )
        .map_err(|_| SessionError::TokenCreation)?;

        env_variables
            .db_connection
            .query(
                &insert(SESSION_TABLE, &SESSION_TABLE_COL, None),
                [token.clone(), profile_id, until.to_rfc3339()],
            )
            .await
            .map_err(SessionError::Db)?;

        Ok(SessionTokenValue(token))
    }
}

#[derive(Deserialize, Serialize)]
pub struct SessionLogoff {
    token: SessionTokenValue,
}

impl SessionLogoff {
    pub async fn clear_session(self, connection: Connection) -> Result<String, SessionError> {
        let profile_name = connection
            .query(
                "DELETE FROM session WHERE token = ?
                RETURNING (SELECT name FROM users WHERE id = session.profile_id);",
                [self.token.to_string()],
            )
            .await
            .map_err(|_| SessionError::TokenDoesntExists)?
            .next()
            .await
            .map_err(SessionError::Db)?
            .ok_or(SessionError::TokenDoesntExists)?
            .get::<String>(0)
            .map_err(|_| SessionError::TokenDoesntExists)?;

        Ok(profile_name)
    }
}
