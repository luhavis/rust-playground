use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://www.naver.com")
        .await?
        .text()
        //.json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", response);

    Ok(())
}
