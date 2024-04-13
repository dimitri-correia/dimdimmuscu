use std::collections::HashMap;

pub async fn add_user() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    dbg!(resp);
    Ok(())
}
