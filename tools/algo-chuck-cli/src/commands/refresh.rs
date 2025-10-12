use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use crate::oauth::refresh_access_token;

/// Handle the refresh command for token renewal
pub async fn handle_refresh_command(_matches: &ArgMatches) -> Result<()> {
    println!("🔄 Schwab OAuth2 Token Refresh");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config(_matches)?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get refresh token from TokenManager
    let refresh_token = token_manager.get_refresh_token()?.ok_or_else(|| {
        anyhow::anyhow!("No refresh token found. Please run 'chuck login' first.")
    })?;

    println!("🔍 Found stored refresh token");
    println!("🔄 Requesting new access token from Schwab...");

    // Use the refresh token to get a new access token
    let token_response = refresh_access_token(&config, &refresh_token).await?;

    println!("✅ New access token received successfully!");

    // Save updated access token using TokenManager
    token_manager.update_access_token(&token_response.access_token, token_response.expires_in)?;
    println!("Updated access token saved securely");

    Ok(())
}
