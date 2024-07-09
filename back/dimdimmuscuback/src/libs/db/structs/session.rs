use chrono::{DateTime, Duration, Utc};
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

        let session_token = SessionToken {
            profile_id: profile_id.clone(),
            until,
        };

        let token = session_token
            .encode(&env_variables.secret_key_session)
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

    pub async fn validate_token(
        token: String,
        env_variables: &EnvVariables,
    ) -> Result<String, SessionError> {
        let row = env_variables
            .db_connection
            .query(
                "SELECT profile_id, until FROM session WHERE token = ?",
                [token],
            )
            .await
            .map_err(SessionError::Db)?
            .next()
            .await
            .map_err(SessionError::Db)?;

        //let profile_id: String = row.unwrap().get(0).map_err(SessionError::Db)?;
        let until: String = row.unwrap().get(1).map_err(SessionError::Db)?;

        Self::validate_until_date(&until)?;

        Ok("profile_id".to_string())
    }

    fn validate_until_date(until: &str) -> Result<(), SessionError> {
        let until = until
            .parse::<DateTime<Utc>>()
            .map_err(|_| SessionError::InvalidDate)?;

        if until < Utc::now() {
            Err(SessionError::TokenExpired)
        } else {
            Ok(())
        }
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
