use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, CredentialsManager, SchwabConfig, TokenManager};

/// Handle the config command and its subcommands
pub async fn handle_config_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("show", _)) => handle_config_show().await,
        Some(("set", sub_matches)) => handle_config_set(sub_matches).await,
        Some(("reset", _)) => handle_config_reset().await,
        _ => unreachable!("Config subcommand is required"),
    }
}

/// Display current configuration
async fn handle_config_show() -> Result<()> {
    println!("🔧 Current Configuration\n");

    let config_manager = ConfigManager::new()?;

    // Load config without validation for display purposes
    let mut config = SchwabConfig::default();

    // Apply file config if it exists
    let config_path = config_manager.config_file_path();
    if config_path.exists() {
        let config_content = std::fs::read_to_string(&config_path)?;
        config = toml::from_str(&config_content)?;
    }

    // Apply environment overrides
    if let Ok(client_id) = std::env::var("SCHWAB_CLIENT_ID") {
        config.client.client_id = Some(client_id);
    }
    // Check client secret from multiple sources
    let mut client_secret_status = "❌ Not configured";
    let mut client_secret_source = "";

    // Check encrypted credentials first
    let credentials_manager = CredentialsManager::new(&config_manager)?;
    if credentials_manager.has_credentials() {
        client_secret_status = "🔐 Configured (encrypted)";
        client_secret_source = " (from encrypted storage)";
    }

    // Environment variable overrides encrypted storage
    if std::env::var("SCHWAB_CLIENT_SECRET").is_ok() {
        client_secret_status = "✅ Configured (env var)";
        client_secret_source = " (from environment variable)";
    }

    println!("Client Configuration:");
    if let Some(client_id) = &config.client.client_id {
        println!("  Client ID: {}...", &client_id[..12.min(client_id.len())]);
    } else {
        println!("  Client ID: ❌ Not configured");
    }

    println!(
        "  Client Secret: {}{}",
        client_secret_status, client_secret_source
    );

    println!("\nAPI Configuration:");
    println!("  Auth URL: {}", config.api.auth_url);
    println!("  Token URL: {}", config.api.token_url);
    println!("  Callback URL: {}", config.api.callback_url);

    println!("\nConfiguration Files:");
    println!("  Config: {}", config_manager.config_file_path().display());

    let token_manager = TokenManager::new(&config_manager)?;
    if token_manager.has_tokens() {
        println!("  Tokens: 🔐 Encrypted and stored securely");
    } else {
        println!("  Tokens: ❌ No tokens found");
    }

    Ok(())
}

/// Set configuration values
async fn handle_config_set(matches: &ArgMatches) -> Result<()> {
    println!("🔧 Setting Configuration\n");

    let config_manager = ConfigManager::new()?;

    // Load config without validation for modification
    let mut config = SchwabConfig::default();

    // Apply file config if it exists
    let config_path = config_manager.config_file_path();
    if config_path.exists() {
        let config_content = std::fs::read_to_string(&config_path)?;
        config = toml::from_str(&config_content)?;
    }

    // Apply environment overrides before CLI modifications
    if let Ok(client_id) = std::env::var("SCHWAB_CLIENT_ID") {
        config.client.client_id = Some(client_id);
    }
    if let Ok(client_secret) = std::env::var("SCHWAB_CLIENT_SECRET") {
        config.client.client_secret = Some(client_secret);
    }

    let mut updated = false;
    let mut credentials_updated = false;

    // Handle client_secret separately (encrypted storage)
    if let Some(client_secret) = matches.get_one::<String>("client-secret") {
        let credentials_manager = CredentialsManager::new(&config_manager)?;
        credentials_manager.set_client_secret(client_secret)?;
        println!("✅ Client Secret encrypted and stored securely");
        credentials_updated = true;
    }

    if let Some(client_id) = matches.get_one::<String>("client-id") {
        config.client.client_id = Some(client_id.clone());
        println!("✅ Client ID updated");
        updated = true;
    }

    if let Some(callback_url) = matches.get_one::<String>("callback-url") {
        config.api.callback_url = callback_url.clone();
        config.parse_callback_url()?; // Update derived fields
        println!("✅ Callback URL updated");
        updated = true;
    }

    if updated {
        config_manager.save_config(&config)?;
        println!("\n🔧 Configuration saved successfully!");
    } else if !credentials_updated {
        println!("❌ No configuration values provided to set");
        println!("Use --client-id, --client-secret, or --callback-url to set values");
    }

    if credentials_updated && !updated {
        println!("\n🔐 Encrypted credentials saved successfully!");
    }

    Ok(())
}

/// Reset configuration to defaults and clear tokens
async fn handle_config_reset() -> Result<()> {
    println!("🔧 Resetting Configuration\n");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let credentials_manager = CredentialsManager::new(&config_manager)?;

    // Clear encrypted credentials first
    if credentials_manager.has_credentials() {
        credentials_manager.clear_credentials()?;
        println!("✅ Cleared encrypted credentials");
    }

    // Clear tokens
    if token_manager.has_tokens() {
        token_manager.clear_tokens()?;
        println!("✅ Cleared all stored tokens");
    }

    // Reset configuration to defaults
    let default_config = SchwabConfig::default();
    config_manager.save_config(&default_config)?;
    println!("✅ Reset configuration to defaults");

    println!("\n🔧 Reset complete! Use 'schwab-authenticator config set' to configure");

    Ok(())
}
