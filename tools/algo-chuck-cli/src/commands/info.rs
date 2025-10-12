use anyhow::Result;
use clap::ArgMatches;

use crate::auth::AutoRefresher;
use crate::config::{ConfigManager, TokenManager};
use crate::display::display_encrypted_token_info;

/// Handle the info command to display token status
pub async fn handle_info_command(matches: &ArgMatches) -> Result<()> {
    println!("🔍 Schwab OAuth2 Token Information\n");

    // Load configuration to create TokenManager
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config(matches)?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Try auto-refresh if enabled and tokens exist
    if token_manager.has_tokens() {
        let auto_refresher = AutoRefresher::new(config.clone(), &config_manager)?;

        println!(
            "🔄 Auto-refresh: {}",
            if auto_refresher.is_enabled() {
                "✅ Enabled"
            } else {
                "❌ Disabled"
            }
        );

        if auto_refresher.is_enabled() {
            if let Err(e) = auto_refresher.ensure_valid_token().await {
                println!("⚠️  Auto-refresh failed: {}", e);
            }
        } else {
            // Show token status without auto-refresh
            match auto_refresher.get_token_status()? {
                crate::auth::auto_refresh::TokenStatus::Expired => {
                    println!("⚠️  Access token is expired - run 'chuck refresh'");
                }
                crate::auth::auto_refresh::TokenStatus::ExpiresSoon => {
                    println!("⚠️  Access token expires soon - consider refreshing");
                }
                _ => {}
            }
        }
        println!();
    }

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
