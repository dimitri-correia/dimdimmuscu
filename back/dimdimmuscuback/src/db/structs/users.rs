use std::fmt::Error;

use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

use crate::errors::auth::signup::SignupError;
use crate::errors::auth::signup::SignupError::{ErrorHashing, ErrorParsingBirthday, ErrorWithDb};

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
    pub birthdate: String,
}

impl UserForCreate {
    pub async fn add_new_user_in_db(self, pool: &PgPool) -> Result<(), SignupError> {
        let utc_date = match DateTime::parse_from_rfc3339(&self.birthdate) {
            Ok(date) => date.with_timezone(&Utc),
            Err(_) => return Err(ErrorParsingBirthday),
        };

        dbg!(&utc_date);

        let user: User = sqlx::query_as(
            "INSERT INTO users (name, birthdate, account_creation) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&self.username)
        .bind(utc_date)
        .bind(Local::now())
        .fetch_one(pool)
        .await
        .map_err(ErrorWithDb)?; // need more precision

        dbg!(&user);

        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = Argon2::default()
            .hash_password(self.pwd_clear.as_ref(), &salt)
            .map_err(ErrorHashing)?
            .to_string();

        dbg!(&password_hash);

        sqlx::query("INSERT INTO users_auth (profile_id, pwd) VALUES ($1, $2)")
            .bind(user.id)
            .bind(password_hash)
            .execute(pool)
            .await
            .map_err(ErrorWithDb)?;

        Ok(())
    }
}

#[derive(Debug, FromRow)]
pub struct UserForLogin {
    pub id: i64,
    pub pwd: String,
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
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
