use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        process::exit(1);
    }
    
    let url = &args[1];
    
    match fetch_url(url).await {
        Ok(content) => {
            println!("Response:");
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("HTTP GET failed: {}", e);
            process::exit(1);
        }
    }
}

async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(format!("{} {}", response.status().as_u16(), response.status().canonical_reason().unwrap_or("Unknown")).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Mock, Server};

    #[tokio::test]
    async fn test_fetch_url_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("Hello, World!")
            .create_async()
            .await;

        let url = format!("{}/test", server.url());
        let result = fetch_url(&url).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_fetch_url_404() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/notfound")
            .with_status(404)
            .with_body("Not Found")
            .create_async()
            .await;

        let url = format!("{}/notfound", server.url());
        let result = fetch_url(&url).await;

        mock.assert_async().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("404"));
    }

    #[tokio::test]
    async fn test_fetch_url_server_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/error")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let url = format!("{}/error", server.url());
        let result = fetch_url(&url).await;

        mock.assert_async().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("500"));
    }

    #[tokio::test]
    async fn test_fetch_url_invalid_url() {
        let result = fetch_url("not-a-valid-url").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_url_with_json_response() {
        let mut server = Server::new_async().await;
        let json_body = r#"{"message": "Hello", "status": "ok"}"#;
        let mock = server
            .mock("GET", "/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .create_async()
            .await;

        let url = format!("{}/json", server.url());
        let result = fetch_url(&url).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json_body);
    }
}