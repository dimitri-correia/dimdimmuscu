use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    _name: String,
    _birthdate: DateTime<Utc>,
    _account_creation: DateTime<Utc>,
}

// impl User {
//     async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
//         let row: Self = sqlx::query_as("SELECT * FROM users WHERE name = $1")
//             .bind(name)
//             .fetch_one(pool)
//             .await
//             .unwrap();
//         Ok(row)
//     }
// }
