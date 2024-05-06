use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use libsql::Connection;
use serde::{Deserialize, Serialize};

use crate::libs::db::{SESSION_TABLE, SESSION_TABLE_COL};
use crate::libs::db::methods::queries::insert;
use crate::libs::env::EnvVariables;
use crate::libs::errors::auth_errors::session_errors::SessionError;
use crate::libs::mw::mw_auth::SessionToken;

#[derive(Deserialize, Serialize)]
pub struct SessionTokenValue(String);

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
            &EncodingKey::from_secret(env_variables.secret_key_session.expose_secret()),
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

    pub fn get(&self) -> String {
        self.0.to_string()
    }

    pub(crate) fn get_associated_user(token: &str, env_variables: &EnvVariables) -> String {
        decode::<SessionToken>(
            token,
            &DecodingKey::from_secret(env_variables.secret_key_session.expose_secret()),
            &Validation::default(),
        )
            .map_err(|_| SessionError::BadToken)
            .map(|data| data.claims.profile_id)
            .unwrap_or("[impossible to decode token]".to_string())
    }
}

#[derive(Deserialize, Serialize)]
pub struct SessionLogoff {
    token: SessionTokenValue,
}

impl SessionLogoff {
    pub fn get_associated_user(&self, env_variables: &EnvVariables) -> String {
        SessionTokenValue::get_associated_user(&self.token.0, env_variables)
    }

    pub async fn clear_session(self, connection: Connection) -> Result<String, SessionError> {
        let profile_name = connection
            .query(
                "DELETE FROM session WHERE token = ?
                RETURNING (SELECT name FROM users WHERE id = session.profile_id);",
                [self.token.0],
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
