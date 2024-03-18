use serde::Deserialize;
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Debug, FromRow)]
pub struct UserForLogin {
    pub id: i32,
    pub pwd: String,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

impl UserForLogin {
    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
        let row = sqlx::query_as::<_, Self>(
            "SELECT * FROM users_auth WHERE profile_id = (SELECT id FROM users WHERE name = $1)",
        )
        .bind(name)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }
}
