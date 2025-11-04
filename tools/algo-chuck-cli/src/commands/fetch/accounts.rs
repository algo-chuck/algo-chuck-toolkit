use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_trader::TraderClient;

/// Handle the account numbers command for data retrieval
pub async fn handle_account_numbers_command(_matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Account Numbers");
    // Implement the logic to fetch account numbers here

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let client = TraderClient::new(reqwest::Client::new());
    let data = client.get_account_numbers(&access_token).await?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the accounts command for data retrieval
pub async fn handle_accounts_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Accounts");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get optional fields parameter
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());

    let client = TraderClient::new(reqwest::Client::new());
    let data = client.get_accounts(&access_token, fields).await?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the account command for data retrieval
pub async fn handle_account_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Account");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required account number parameter
    let account_number = matches
        .get_one::<String>("account-number")
        .ok_or_else(|| anyhow::anyhow!("Account number is required"))?;

    // Get optional fields parameter
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());

    let client = TraderClient::new(reqwest::Client::new());
    let data = client
        .get_account(&access_token, account_number, fields)
        .await?;
    println!("{:#?}", data);

    Ok(())
}
