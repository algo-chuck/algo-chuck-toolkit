//! Example demonstrating both async and sync OAuth clients
//!
//! This example shows how to use both the AsyncOAuthClient and SyncOAuthClient
//! to authenticate with the Schwab API.

use schwab_api_oauth::{AsyncOAuthClient, OAuthConfig, SyncOAuthClient};

/// Example using the async OAuth client
async fn async_example() {
    let config = OAuthConfig::new(
        "your_client_id".to_string(),
        "your_client_secret".to_string(),
        "https://localhost:8182/callback".to_string(),
    );

    let client = AsyncOAuthClient::new(reqwest::Client::new(), config);

    // Build authorization URL
    let state = "random_state_string";
    match client.build_auth_url(state) {
        Ok(auth_url) => {
            println!("Async: Visit this URL to authorize:");
            println!("{}", auth_url);

            // In a real application, you would:
            // 1. Redirect user to auth_url
            // 2. Receive the authorization code from the callback
            // 3. Exchange the code for tokens

            // Example token exchange (commented out since we don't have a real code):
            // let tokens = client.exchange_code_for_token("authorization_code").await.unwrap();
            // println!("Access token: {}", tokens.access_token);

            // Example token refresh:
            // let refreshed = client.refresh_access_token(&tokens.refresh_token).await.unwrap();
            // println!("Refreshed access token: {}", refreshed.access_token);
        }
        Err(e) => eprintln!("Error building auth URL: {}", e),
    }
}

/// Example using the sync OAuth client
fn sync_example() {
    let config = OAuthConfig::new(
        "your_client_id".to_string(),
        "your_client_secret".to_string(),
        "https://localhost:8182/callback".to_string(),
    );

    let client = SyncOAuthClient::new(ureq::Agent::new(), config);

    // Build authorization URL
    let state = "random_state_string";
    match client.build_auth_url(state) {
        Ok(auth_url) => {
            println!("\nSync: Visit this URL to authorize:");
            println!("{}", auth_url);

            // In a real application, you would:
            // 1. Redirect user to auth_url
            // 2. Receive the authorization code from the callback
            // 3. Exchange the code for tokens

            // Example token exchange (commented out since we don't have a real code):
            // let tokens = client.exchange_code_for_token("authorization_code").unwrap();
            // println!("Access token: {}", tokens.access_token);

            // Example token refresh:
            // let refreshed = client.refresh_access_token(&tokens.refresh_token).unwrap();
            // println!("Refreshed access token: {}", refreshed.access_token);
        }
        Err(e) => eprintln!("Error building auth URL: {}", e),
    }
}

#[tokio::main]
async fn main() {
    println!("=== OAuth Client Examples ===\n");

    // Run async example
    async_example().await;

    // Run sync example
    sync_example();

    println!("\n=== Examples Complete ===");
}
