use std::env;
use dotenv::dotenv;
use reqwest::Error;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";

async fn get_request() -> Result<(), Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("$GEMINI_API_KEY is not set");
    let url = format!("{BASE_URL}{api_key}");
    let json_data = r#"{"contents":[{"parts":[{"text":"Write a story about a magic backpack"}]}]}"#;

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    get_request().await?;
    Ok(())
}
