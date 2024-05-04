use std::collections::HashMap;

use crate::get_address;

pub async fn delete_user(token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("token", token);

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/connect/delete_user", get_address()))
        .json(&map)
        .send()
        .await?;

    dbg!(res.status());
    Ok(())
}
