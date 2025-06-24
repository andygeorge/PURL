use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).ok_or_else(|| {
        "Usage: program_name <URL>\n"
    })?;

    let http = reqwest::Client::new();
    let response = http.get(&url).send().await?;

    if response.status().is_success() {
        println!("Response:\n{}", response.text().await?);
    } else {
        eprintln!("HTTP GET failed: {} {}", response.status(), response.status()); // More idiomatic error printing
        // You could also extract the reason phrase, but it's often less useful.
        // let reason = response.status();
        // eprintln!("HTTP GET failed: {} {}", response.status(), reason);
        std::process::exit(1); // Exit with an error code
    }

    Ok(())
}