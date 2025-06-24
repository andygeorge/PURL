---

# PURL - HTTP GET Client

A simple HTTP GET client written in Rust using the `reqwest` crate.

## Usage

To run the program, provide a URL as an argument:

```sh
cargo run -- PURL <URL>
```

## Example

```sh
cargo run -- PURL https://example.com
```

## Code Explanation

The following sections explain the code in detail.

### Imports

```rust
use reqwest::Error;
use std::env;
```

- `reqwest::Error`: Import the error type from the `reqwest` crate.
- `std::env`: Import the environment module to handle command-line arguments.

### Main Function

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = env::args().nth(1).expect("Usage: PURL <URL>");
```

- `#[tokio::main]`: Mark the `main` function as an asynchronous entry point using Tokio.
- `let url = env::args().nth(1).expect("Usage: PURL <URL>");`: Retrieve the URL from command-line arguments and handle cases where no argument is provided.

### Making HTTP Request

```rust
    let response = reqwest::get(&url).await?;
```

- `reqwest::get(&url).await?`: Make an asynchronous GET request to the provided URL. The `await` keyword waits for the request to complete, and `?` propagates any errors.

### Handling Response

```rust
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
```

- `if response.status().is_success()`: Check if the HTTP status code indicates a successful request.
  - If true, print the response text.
- `else`: Handle non-successful requests.
  - Use `match` to handle the `Option<&str>` returned by `canonical_reason()`.
    - If some, print the status and reason.
    - If none, print only the status.

### Error Handling

The function returns a `Result<(), Error>`, ensuring proper error handling throughout the program.

---

This documentation provides an overview of the `main.rs` file, its usage, and a detailed explanation of the code.
