// use sqlx::{Error, PgPool};
//
// #[derive(Debug, sqlx::FromRow)]
// pub struct Muscle {
//     id: i32,
//     name: String,
// }
//
// impl Muscle {
//     async fn create_new(pool: &PgPool, name: &str) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>("INSERT INTO muscles (name) VALUES ($1)")
//             .bind(name)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_all(pool: &PgPool) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM muscles")
//             .fetch_all(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM muscles WHERE name = $1")
//             .bind(name)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM muscles WHERE id = $1")
//             .bind(id)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
// }
