use reqwest::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = env::args().nth(1).expect("Usage: PURL <URL>");
    
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("Response:\n{}", response.text().await?);
    } else {
        match response.status().canonical_reason() {
            Some(reason) => eprintln!("HTTP GET failed: {} {}", response.status(), reason),
            None => eprintln!("HTTP GET failed: {}", response.status()),
        }
    }

    Ok(())
}