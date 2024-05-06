use std::collections::HashMap;

use crate::get_address;

pub async fn add_muscle(token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut map_create = HashMap::new();
    map_create.insert("username", "username");
    map_create.insert("pwd_clear", "pwd");
    map_create.insert("birthdate", "Utc");

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/muscles/add", get_address()))
        .header("Authorization", format!("Bearer {}", token))
        .json(&map_create)
        .send()
        .await?;

    dbg!(res.status());
    Ok(())
}
