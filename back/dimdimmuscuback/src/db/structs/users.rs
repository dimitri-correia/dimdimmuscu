use sqlx::{Error, PgPool};

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    birthdate: String,
}

impl User {
    async fn create_new(pool: &PgPool, name: &str, birthdate: &str) -> Result<Self, Error> {
        let row = sqlx::query_as::<_, Self>(
            "INSERT INTO users (name, birthdate) VALUES ($1, $2)").bind(name).bind(birthdate)
            .fetch_one(pool)
            .await?;
        Ok(row)
    }

    async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
        let row = sqlx::query_as::<_, Self>(
            "SELECT * FROM users WHERE name = $1"
        ).bind(name)
            .fetch_one(pool)
            .await?;
        Ok(row)
    }
}