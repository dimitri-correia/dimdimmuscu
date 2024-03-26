use std::fmt::Error;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    name: String,
    birthdate: DateTime<Utc>,
    account_creation: DateTime<Utc>,
}

impl User {
    async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
        let row: Self = sqlx::query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
            .unwrap();
        Ok(row)
    }
}
