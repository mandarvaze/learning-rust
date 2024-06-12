use std::env;
use dotenv::dotenv;
use reqwest::Error;
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    /// The prompt for Gemini
    prompt: String,
}

fn extract_text_from_json(json_body: &str) -> Option<String> {
    let v: Value = serde_json::from_str(json_body).unwrap();

    if let Some(parts) = v["candidates"][0]["content"]["parts"].as_array() {
        if let Some(text) = parts[0]["text"].as_str() {
            return Some(text.to_string());
        }
    }
    None
}

async fn perform_request(client: &reqwest::Client, url: &str, json_data: &str) -> Result<reqwest::Response, reqwest::Error> {
    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
}

async fn get_response_body(response: reqwest::Response) -> Result<String, reqwest::Error> {
    response.text().await
}

async fn get_result(prompt: &str) -> Result<String, Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("$GEMINI_API_KEY is not set");
    let base_url = env::var("BASE_URL").expect("$BASE_URL is not set");
    let url = format!("{base_url}?key={api_key}");
    let json_data = format!(r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#, prompt);
    let client = reqwest::Client::new();

    let response = perform_request(&client, &url, &json_data).await?;

    #[cfg(debug_assertions)] 
    println!("Status: {}", response.status());

    let body = get_response_body(response).await?;
    let answer: String = extract_text_from_json(&body).unwrap();   

    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    dotenv().ok();
    let answer = get_result(&args.prompt).await?;
    println!("Answer:\n{}", answer);
    Ok(())
}
