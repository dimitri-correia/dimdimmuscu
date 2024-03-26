use chrono::{DateTime, Local, Utc};
use sqlx::{FromRow, PgPool};

use crate::errors::cookies::CookieError;

#[derive(Debug, FromRow)]
pub struct Session {
    token: String,
    id: i64,
    until: DateTime<Utc>,
}

impl Session {
    pub async fn new(profile_id: i64, pool: &PgPool) -> Result<String, CookieError> {
        let token_for_auth = String::new();

        sqlx::query("INSERT INTO session (token, profile_id, until) VALUES ($1, $2, $3)")
            .bind(&token_for_auth)
            .bind(profile_id)
            .bind(Local::now())
            .execute(pool)
            .await
            .map_err(CookieError::ErrorWithDb)?; // need more precision;

        Ok(token_for_auth)
    }
}
