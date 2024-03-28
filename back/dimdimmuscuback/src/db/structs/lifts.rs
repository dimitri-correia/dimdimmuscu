// use sqlx::{Error, PgPool};
//
// #[derive(Debug, sqlx::FromRow)]
// pub struct Lifts {
//     id: i32,
//     profile_id: i32,
//     movement_id: i32,
//     lift_time: String,
// }
//
// impl Lifts {
//     async fn create_new(
//         pool: &PgPool,
//         id: i32,
//         profile_id: i32,
//         movement_id: i32,
//         lift_time: String,
//     ) -> Result<Self, Error> {
//         let row = sqlx::query_as::<_, Self>(
//             "INSERT INTO lifts (id, profile_id, movement_id, lift_time) VALUES ($1, $2, $3, $4)",
//         )
//         .bind(id)
//         .bind(profile_id)
//         .bind(movement_id)
//         .bind(lift_time)
//         .fetch_one(pool)
//         .await?;
//         Ok(row)
//     }
//
//     async fn find_by_profile_id(pool: &PgPool, profile_id: i32) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>("SELECT * FROM muscles WHERE profile_id = $1")
//             .bind(profile_id)
//             .fetch_all(pool)
//             .await?;
//         Ok(row)
//     }
//
//     async fn find_by_profile_id_and_movement_id(
//         pool: &PgPool,
//         profile_id: i32,
//         movement_id: i32,
//     ) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>(
//             "SELECT * FROM muscles WHERE (profile_id, movement_id) = ($1, $2)",
//         )
//         .bind(profile_id)
//         .bind(movement_id)
//         .fetch_all(pool)
//         .await?;
//         Ok(row)
//     }
//
//     async fn find_by_profile_id_and_lift_time(
//         pool: &PgPool,
//         profile_id: i32,
//         lift_time: i32,
//     ) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>(
//             "SELECT * FROM muscles WHERE (profile_id, lift_time) = ($1, $2)",
//         )
//         .bind(profile_id)
//         .bind(lift_time)
//         .fetch_all(pool)
//         .await?;
//         Ok(row)
//     }
//
//     async fn find_by_profile_id_movement_id_and_lift_time(
//         pool: &PgPool,
//         profile_id: i32,
//         movement_id: i32,
//         lift_time: i32,
//     ) -> Result<Vec<Self>, Error> {
//         let row = sqlx::query_as::<_, Self>(
//             "SELECT * FROM muscles WHERE (profile_id, movement_id, lift_time) = ($1, $2, $3)",
//         )
//         .bind(profile_id)
//         .bind(movement_id)
//         .bind(lift_time)
//         .fetch_all(pool)
//         .await?;
//         Ok(row)
//     }
// }
