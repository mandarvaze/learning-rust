use std::env;
use dotenv::dotenv;
use reqwest::Error;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The prompt for Gemini
    prompt: String,
}

async fn get_result(prompt: &str) -> Result<(), Error> {
    let api_key = env::var("GEMINI_API_KEY").expect("$GEMINI_API_KEY is not set");
    let base_url = env::var("BASE_URL").expect("$BASE_URL is not set");
    let url = format!("{base_url}?key={api_key}");
    let json_data = format!(r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#, prompt);
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
    let args = Cli::parse();
    dotenv().ok();
    get_result(&args.prompt).await?;
    Ok(())
}
