use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use crate::display::display_encrypted_token_info;

/// Handle the info command to display token status
pub async fn handle_info_command(_matches: &ArgMatches) -> Result<()> {
    println!("🔍 Schwab OAuth2 Token Information\n");

    // Load configuration to create TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Check TokenManager tokens
    if token_manager.has_tokens() {
        println!("🔐 Secure Token Storage:");
        match token_manager.get_tokens_info() {
            Ok(tokens) => {
                display_encrypted_token_info(
                    "Access Token",
                    &tokens.access_token,
                    &tokens.access_token_expiry,
                );
                display_encrypted_token_info(
                    "Refresh Token",
                    &tokens.refresh_token,
                    &tokens.refresh_token_expiry,
                );
            }
            Err(e) => {
                println!("❌ Failed to load secure tokens: {}", e);
            }
        }
        println!();
    } else {
        println!("🔐 Secure Token Storage: ❌ No tokens found");
        println!("Run 'chuck login' to authenticate first.\n");
    }

    Ok(())
}
