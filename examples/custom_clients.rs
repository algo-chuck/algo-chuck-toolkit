//! Example demonstrating different HTTP client options with the Schwab API
//!
//! This example shows:
//! 1. Using reqwest (async) - the default
//! 2. Using ureq (sync/blocking)  
//! 3. Different ownership patterns (owned, borrowed, Arc)
//!
//! Note: This example demonstrates the API structure but won't run without valid credentials.

// Example 1: Using reqwest (async) with different ownership patterns
async fn example_reqwest_async() {
    use schwab_api_trader::AsyncTraderClient;
    use std::sync::Arc;

    let access_token = "your_access_token".to_string();

    // Option A: Owned client (takes ownership)
    let client = reqwest::Client::new();
    let trader = AsyncTraderClient::new(client);

    // Option B: Borrowed client (useful in server contexts)
    let client = reqwest::Client::new();
    let trader = AsyncTraderClient::new(&client);
    // client can still be used elsewhere

    // Option C: Arc-wrapped client (explicit shared ownership)
    let client = Arc::new(reqwest::Client::new());
    let trader = AsyncTraderClient::new(Arc::clone(&client));
    // client can be cloned and shared across threads
}

// Example 2: Server-style usage with shared async client
async fn server_example() {
    use schwab_api_trader::AsyncTraderClient;
    use std::sync::Arc;

    // Create a shared client that will be used across many requests
    let shared_client = Arc::new(reqwest::Client::new());

    // Each request handler can clone the Arc and create its own trader client
    let client_clone = Arc::clone(&shared_client);
    let trader = AsyncTraderClient::new(client_clone);

    // The Arc allows efficient sharing without copying the underlying client
}

// Example 3: Custom HTTP client implementation
// You can implement AsyncHttpClient or SyncHttpClient for your own types
mod custom {
    use async_trait::async_trait;
    use http::{Request, Response};
    use schwab_api_core::{AsyncHttpClient, HttpError};

    // Example: Custom client wrapper with logging
    pub struct LoggingClient {
        inner: reqwest::Client,
    }

    impl LoggingClient {
        pub fn new() -> Self {
            Self {
                inner: reqwest::Client::new(),
            }
        }
    }

    #[async_trait]
    impl AsyncHttpClient for LoggingClient {
        type Error = HttpError;

        async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
            println!("Executing request to: {:?}", request.uri());

            // Delegate to the underlying reqwest client's implementation
            self.inner.execute(request).await
        }
    }
}

fn main() {
    println!("This example demonstrates the HTTP client options available.");
    println!("See the source code for usage patterns including:");
    println!("  - Owned, borrowed, and Arc-wrapped clients");
    println!("  - Server-style shared client usage");
    println!("  - Custom client implementations");
    println!("\nThe core crate supports both async (reqwest) and sync (ureq) clients.");
    println!("Enable different clients with feature flags in schwab-api-core.");
}
