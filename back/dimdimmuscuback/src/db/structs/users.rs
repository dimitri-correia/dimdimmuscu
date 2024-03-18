use std::fmt::Error;

use chrono::Local;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use lib_auth::pwd;
use lib_auth::pwd::ContentToHash;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    birthdate: String,
    account_creation: String,
}

impl User {
    async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
        let row = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(row)
    }
}

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
    pub birthdate: String,
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

impl UserForCreate {
    pub async fn add(pool: &PgPool, user_for_create: Self) -> Result<UserForLogin, Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, birthdate, account_creation) VALUES ($1, $2, $3)",
        )
        .bind(user_for_create.username)
        .bind(user_for_create.birthdate)
        .bind(Local::now().to_rfc3339())
        .fetch_one(pool)
        .await?;

        let pwd_salt = "";
        let encoded_pwd = encode_pwd(&user_for_create.pwd_clear, pwd_salt).await;
        let token_salt = "";
        let auth_user = sqlx::query_as::<_, UserForLoginInDb>(
            "INSERT INTO users_auth (profile_id, pwd, pwd_salt, token_salt) VALUES ($1, $2, $3, $4)")
            .bind(user.id)
            .bind(encoded_pwd)
            .bind(pwd_salt)
            .bind(token_salt)
            .fetch_one(pool)
            .await?;

        UserForLogin {}
    }
}

async fn encode_pwd(pwd_clear: &String, salt: Uuid) -> Result<(), Error> {
    let pwd = pwd::hash_pwd(ContentToHash {
        content: pwd_clear.to_string(),
        salt,
    })
    .await
    .map_err(|_| Error)?;
    Ok(())
}

impl UserForLogin {
    pub async fn update_pwd(
        pool: &PgPool,
        username: &String,
        pwd_clear: &String,
    ) -> Result<Self, Error> {
        let user: Self = Self::find_by_name(pool, username).await?;
        encode_pwd(pwd_clear, user.pwd_salt).await?;

        UserForLogin {}
    }

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
