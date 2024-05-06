// use sqlx::{Error, PgPool};
//
// #[derive(Debug, sqlx::FromRow)]
// pub struct Movement {
//     id: i32,
//     name: String,
//     first_muscle_id: i32,
//     second_muscle_id: i32,
//     third_muscle_id: i32,
// }
//
// impl Movement {
//     async fn create_new(
//         pool: &PgPool,
//         name: &str,
//         first_muscle_id: i32,
//         second_muscle_id: i32,
//         third_muscle_id: i32,
//     ) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>(
//             "INSERT INTO movements (name, first_muscle, second_muscle, third_muscle) VALUES ($1, $2, $3, $4)")
//             .bind(name).bind(first_muscle_id).bind(second_muscle_id).bind(third_muscle_id)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_all(pool: &PgPool) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM movements")
//             .fetch_all(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_by_name(pool: &PgPool, name: &str) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM movements WHERE name = $1")
//             .bind(name)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM movements WHERE id = $1")
//             .bind(id)
//             .fetch_one(pool)
//             .await?;
//         Ok(row)
//     }
// }
