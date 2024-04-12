use std::string::ToString;

use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use libsql::{Connection, Rows};
use serde::Deserialize;

use crate::db::methods::queries::insert;
use crate::errors::auth::login::LoginError;
use crate::errors::auth::signup::SignupError;

const USERS_TABLE: &str = "users";
const USERS_TABLE_COL: [&str; 4] = ["id", "name", "birthdate", "account_creation"];
const USERS_AUTH_TABLE: &str = "users_auth";
const USERS_AUTH_TABLE_COL: [&str; 2] = ["profile_id", "pwd"];

#[derive(Deserialize)]
pub struct UserForCreate {
    username: String,
    pwd_clear: String,
    birthdate: String,
}

impl UserForCreate {
    pub async fn add_new_user_in_db(self, conn: &Connection) -> Result<(), SignupError> {
        //todo
        // actually deal with errors

        let utc_date = match DateTime::parse_from_rfc3339(&self.birthdate) {
            Ok(date) => date.with_timezone(&Utc),
            Err(_) => return Err(SignupError::ErrorParsingBirthday),
        };

        let mut user_rows: Rows = conn
            .query(
                &insert(
                    USERS_TABLE,
                    &USERS_TABLE_COL[1..],
                    Some(vec![USERS_TABLE_COL[0]]),
                ),
                [
                    self.username,
                    utc_date.to_rfc3339(),
                    Utc::now().to_rfc3339(),
                ],
            )
            .await
            .map_err(SignupError::ErrorWithDb)?;

        let profile_id = Self::get_profile_id(&mut user_rows).await?;

        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = Argon2::default()
            .hash_password(self.pwd_clear.as_ref(), &salt)
            .map_err(SignupError::ErrorHashing)?
            .to_string();

        conn.query(
            &insert(USERS_AUTH_TABLE, &USERS_AUTH_TABLE_COL, None),
            [profile_id.to_string(), password_hash],
        )
        .await
        .map_err(SignupError::ErrorWithDb)?;

        Ok(())
    }

    async fn get_profile_id(user_rows: &mut Rows) -> Result<i32, SignupError> {
        let user = user_rows.next().await.map_err(SignupError::ErrorWithDb)?;

        let row = match user {
            Some(row) => row,
            None => return Err(SignupError::ErrorParsingBirthday),
        };

        let profile_id: i32 = row
            .get::<i32>(0)
            .map_err(|_| SignupError::ErrorParsingBirthday)?;

        Ok(profile_id)
    }
}

#[derive(Deserialize)]
pub struct UserForLogin {
    username: String,
    pwd_clear: String,
}

impl UserForLogin {
    pub async fn authenticate(self, connection: &Connection) -> Result<i32, LoginError> {
        let mut rows: Rows = connection
            .query(
                "SELECT ua.pwd, ua.profile_id
             FROM users u
             JOIN users_auth ua
             ON u.id = ua.profile_id
             WHERE u.name = $1;",
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
        let user = user_rows.next().await.map_err(LoginError::ErrorWithDb)?;

        let row = match user {
            Some(row) => row,
            None => return Err(LoginError::ErrorWithHash),
        };

        let pwd: String = row
            .get::<String>(0)
            .map_err(|_| LoginError::ErrorWithHash)?;

        let profile_id: i32 = row.get::<i32>(1).map_err(|_| LoginError::ErrorWithHash)?;

        Ok((pwd, profile_id))
    }
}
