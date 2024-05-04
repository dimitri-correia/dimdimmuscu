use std::collections::HashMap;

use chrono::Utc;
use rand::Rng;

use crate::get_address;

pub async fn add_user() -> Result<(String, String), Box<dyn std::error::Error>> {
    let username = format!("hand_testing_{}", rand::rngs::OsRng.gen::<u32>());
    let pwd = "pwd_clear";

    dbg!(&username);

    let mut map_create = HashMap::new();
    map_create.insert("username", username.clone());
    map_create.insert("pwd_clear", pwd.to_string());
    map_create.insert("birthdate", Utc::now().to_rfc3339());

    let client = reqwest::Client::new();
    let res_creation = client
        .post(format!("{}/connect/signup", get_address()))
        .json(&map_create)
        .send()
        .await?;

    dbg!(res_creation.status());

    let mut map_auth = HashMap::new();
    map_auth.insert("username", username.clone());
    map_auth.insert("pwd_clear", pwd.to_string());

    let res_auth = client
        .post(format!("{}/connect/login", get_address()))
        .json(&map_auth)
        .send()
        .await?;

    dbg!(res_auth.status());
    let token = dbg!(res_auth.text().await).expect("Should have the token");

    Ok((username, token.to_string()))
}
