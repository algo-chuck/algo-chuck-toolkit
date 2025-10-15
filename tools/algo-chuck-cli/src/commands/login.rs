use anyhow::Result;
use clap::ArgMatches;

use crate::ca::{CaManager, installer};
use crate::config::{ConfigManager, SchwabConfig, TokenManager};
use crate::oauth::{build_schwab_auth_url, exchange_code_for_token, generate_state};
use crate::server::start_callback_server;

/// Handle the login command for OAuth2 authentication
pub async fn handle_login_command(_matches: &ArgMatches) -> Result<()> {
    println!("🔐 Algo Chuck CLI - Login");

    // Setup Certificate Authority for seamless HTTPS
    let ca_manager = CaManager::new()?;
    if !ca_manager.ca_exists() {
        println!("\n🔐 Setting up Certificate Authority for seamless HTTPS...");

        // Generate CA certificate
        ca_manager.generate_ca().await?;

        // Prompt user to install CA in system trust store
        if installer::prompt_ca_installation()? {
            ca_manager.install_system_ca().await?;
            println!("✅ Certificate Authority installed successfully!");
            println!("   Future OAuth logins will not show certificate warnings.");
        } else {
            eprintln!("⚠️  CA not installed in system trust store.");
            eprintln!("   You may see certificate warnings during OAuth flow.");
            eprintln!("   Run 'chuck ca install' later to eliminate warnings.");
        }
        println!();
    }

    // Load configuration using ConfigManager
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config(_matches)?;

    let client_id = config
        .client
        .client_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client ID not configured"))?;

    // Display client ID safely (handle short IDs)
    if client_id.len() > 12 {
        println!(
            "🔑 Using Client ID: {}...{}",
            &client_id[..6],
            &client_id[client_id.len() - 6..]
        );
    } else {
        println!("🔑 Using Client ID: {}", client_id);
    }

    println!("🌐 Callback URL: {}", SchwabConfig::CALLBACK_URL);

    // Generate state parameter and build authorization URL
    let state = generate_state();
    let auth_url = build_schwab_auth_url(&client_id, &state)?;

    println!("\n🚀 Opening browser for Schwab authentication...");
    if let Err(e) = webbrowser::open(&auth_url) {
        eprintln!("Failed to open browser automatically: {}", e);
        println!("Please manually open this URL in your browser:");
        println!("{}", auth_url);
    }

    // Start callback server and wait for authorization
    let (code, returned_state) = start_callback_server(&config).await?;

    // Verify state parameter
    if returned_state != state {
        return Err(anyhow::anyhow!(
            "State parameter mismatch - possible CSRF attack"
        ));
    }

    println!("🔄 Exchanging authorization code for tokens...");

    // Exchange code for tokens
    let token_response = exchange_code_for_token(&config, &code).await?;

    println!("✅ Tokens received successfully!");

    // Save tokens using TokenManager
    let token_manager = TokenManager::new(&config_manager)?;
    token_manager.save_oauth_tokens(
        &token_response.access_token,
        &token_response.refresh_token,
        token_response.expires_in,
    )?;

    println!("🔐 Tokens saved securely using encryption");
    println!("\n🎉 Authentication complete!");
    println!("💡 Use 'chuck info' to view token status");
    println!("💡 Use 'chuck refresh' to refresh tokens as needed");

    Ok(())
}
