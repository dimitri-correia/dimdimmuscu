use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::structs::users::User;
use crate::errors::auth::login::LoginError;
use crate::errors::auth::signup::SignupError;

#[derive(Deserialize)]
pub struct UserForCreate {
    username: String,
    pwd_clear: String,
    birthdate: String,
}

impl UserForCreate {
    pub async fn add_new_user_in_db(self, pool: &PgPool) -> Result<(), SignupError> {
        let utc_date = match DateTime::parse_from_rfc3339(&self.birthdate) {
            Ok(date) => date.with_timezone(&Utc),
            Err(_) => return Err(SignupError::ErrorParsingBirthday),
        };

        let user: User = sqlx::query_as(
            "INSERT INTO users (name, birthdate, account_creation) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&self.username)
        .bind(utc_date)
        .bind(Local::now())
        .fetch_one(pool)
        .await
        .map_err(SignupError::ErrorWithDb)?; // need more precision

        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = Argon2::default()
            .hash_password(self.pwd_clear.as_ref(), &salt)
            .map_err(SignupError::ErrorHashing)?
            .to_string();

        sqlx::query("INSERT INTO users_auth (profile_id, pwd) VALUES ($1, $2)")
            .bind(user.id)
            .bind(password_hash)
            .execute(pool)
            .await
            .map_err(SignupError::ErrorWithDb)?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct UserForLogin {
    username: String,
    pwd_clear: String,
}

impl UserForLogin {
    pub async fn authenticate(self, pool: &PgPool) -> Result<i64, LoginError> {
        let (password_hash, profile_id): (String, i64) = sqlx::query_as(
            "SELECT ua.pwd, ua.profile_id
             FROM users u
             JOIN users_auth ua ON u.id = ua.profile_id
             WHERE u.name = $1;",
        )
        .bind(&self.username)
        .fetch_one(pool)
        .await
        .map_err(LoginError::ErrorWithDb)?;

        let parsed_hash =
            PasswordHash::new(&password_hash).map_err(|_| LoginError::ErrorWithHash)?;

        Argon2::default()
            .verify_password(self.pwd_clear.as_ref(), &parsed_hash)
            .map_err(|_| LoginError::WrongPwd)?;

        Ok(profile_id)
    }
}
