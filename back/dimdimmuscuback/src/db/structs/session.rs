use std::i32;

use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use libsql::Connection;
use serde::{Deserialize, Serialize};

use crate::errors::auth::session::SessionError;

#[derive(Debug)]
pub struct Session {
    token: String,
    profile_id: i32,
    until: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionToken {
    profile_id: i32,
    until: DateTime<Utc>,
}

impl Session {
    pub async fn create(profile_id: i32, conn: &Connection) -> Result<String, SessionError> {
        let until = Utc::now(); //+ Duration::hours(session_duration_hours);

        let session_token = SessionToken { profile_id, until };

        let token = encode(
            &Header::default(),
            &session_token,
            &EncodingKey::from_secret(b"your_secret_key"),
        )
        .map_err(|_| SessionError::TokenCreation)?;

        // sqlx::query("INSERT INTO session (token, profile_id, until) VALUES ($1, $2, $3)")
        //     .bind(&token)
        //     .bind(profile_id)
        //     .bind(until)
        //     .execute(pool)
        //     .await
        //     .map_err(CookieError::ErrorWithDb)?; // need more precision;

        Ok(token)
    }
}

#[derive(Debug)]
pub struct SessionLogoff {
    token: String,
}

impl SessionLogoff {
    pub async fn clear_session(self, connection: Connection) {}
}
