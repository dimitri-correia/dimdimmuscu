use std::fmt::Error;

use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Debug)]
pub struct UserForLogin {
    pub id: i32,
    pub pwd: String,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Debug, FromRow)]
pub struct UserForLoginInDb {
    pub id: i32,
    pub pwd: String,
    pub pwd_salt: String,
    pub token_salt: String,
}

impl UserForLogin {
    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
        let row = sqlx::query_as::<_, UserForLoginInDb>(
            "SELECT * FROM users_auth WHERE profile_id = (SELECT id FROM users WHERE name = $1)",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|_| Error)?;

        UserForLogin::from(row)
    }

    fn from(user: UserForLoginInDb) -> Result<Self, Error> {
        let pwd_salt = Uuid::parse_str(&user.pwd_salt).map_err(|_| Error)?;
        let token_salt = Uuid::parse_str(&user.token_salt).map_err(|_| Error)?;

        Ok(UserForLogin {
            id: user.id,
            pwd: user.pwd,
            pwd_salt,
            token_salt,
        })
    }
}
