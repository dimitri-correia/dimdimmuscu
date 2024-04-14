use std::string::ToString;

use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use libsql::{Connection, Rows};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::methods::queries::insert;
use crate::db::{USERS_AUTH_TABLE, USERS_AUTH_TABLE_COL, USERS_TABLE, USERS_TABLE_COL};
use crate::errors::auth::login_logoff::{LoginError, LogoffError};
use crate::errors::auth::signup::SignupError;

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pwd_clear: String,
    birthdate: String,
}

impl UserForCreate {
    pub async fn add_new_user_in_db(self, conn: &Connection) -> Result<(), SignupError> {
        //todo
        // actually deal with errors

        let uuid = Uuid::now_v7().to_string();

        let utc_date = match DateTime::parse_from_rfc3339(&self.birthdate) {
            Ok(date) => date.with_timezone(&Utc),
            Err(_) => return Err(SignupError::ErrorParsingBirthday),
        };

        conn.execute(
            &insert(USERS_TABLE, &USERS_TABLE_COL, None),
            [
                uuid.clone(),
                self.username,
                utc_date.to_rfc3339(),
                Utc::now().to_rfc3339(),
            ],
        )
        .await
        .map_err(SignupError::ErrorWithDb)?;

        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = Argon2::default()
            .hash_password(self.pwd_clear.as_ref(), &salt)
            .map_err(SignupError::ErrorHashing)?
            .to_string();

        conn.query(
            &insert(USERS_AUTH_TABLE, &USERS_AUTH_TABLE_COL, None),
            [uuid, password_hash],
        )
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
    pub async fn authenticate(self, connection: &Connection) -> Result<i32, LoginError> {
        let mut rows = connection
            .query(
                "SELECT ua.pwd, ua.profile_id
             FROM users u
             JOIN users_auth ua
             ON u.id = ua.profile_id
             WHERE u.name = ?;",
                [self.username],
            )
            .await
            .map_err(LoginError::ErrorWithDb)?;

        let (pwd_in_db, profile_id) = Self::get_info_from_db(&mut rows).await?;

        let parsed_hash = PasswordHash::new(&pwd_in_db).map_err(|_| LoginError::ErrorWithHash)?;

        Argon2::default()
            .verify_password(self.pwd_clear.as_ref(), &parsed_hash)
            .map_err(|_| LoginError::WrongPwd)?;

        Ok(profile_id)
    }

    async fn get_info_from_db(user_rows: &mut Rows) -> Result<(String, i32), LoginError> {
        let row = user_rows
            .next()
            .await
            .map_err(LoginError::ErrorWithDb)?
            .ok_or(LoginError::ErrorWithHash)?;

        let pwd: String = row
            .get::<String>(0)
            .map_err(|_| LoginError::ErrorWithHash)?;

        let profile_id: i32 = row.get::<i32>(1).map_err(|_| LoginError::ErrorWithHash)?;

        Ok((pwd, profile_id))
    }
}

#[derive(Deserialize)]
pub struct UserForDelete {
    token: String,
}

impl UserForDelete {
    pub async fn delete_user(self, connection: Connection) -> Result<String, LogoffError> {
        let row = connection
            .query(
                "SELECT u.name, u.id
                FROM session s
                JOIN users u ON s.profile_id = u.id
                WHERE s.token = ?;",
                [self.token],
            )
            .await
            .map_err(LogoffError::ErrorWithDb)?
            .next()
            .await
            .map_err(LogoffError::ErrorWithDb)?
            .ok_or(LogoffError::NotConnected)?;
        let profile_name = row
            .get::<String>(0)
            .map_err(|_| LogoffError::NotConnected)?;
        let profile_id = row.get::<i32>(1).map_err(|_| LogoffError::NotConnected)?;

        connection
            .execute("DELETE FROM users WHERE id = ?;", [profile_id])
            .await
            .map_err(LogoffError::ErrorWithDb)?;

        Ok(profile_name)
    }
}
