use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    _name: String,
    _birthdate: DateTime<Utc>,
    _account_creation: DateTime<Utc>,
}

// impl User {
//     pub async fn get_myself(id: i64, conn: &Connection) -> Self {
//         let mut rows = conn
//             .query(
//                 "SELECT *
//              FROM users u
//              WHERE u.id = ?;",
//                 [id],
//             )
//             .await;
//             // .map_err();
//     }
// }
