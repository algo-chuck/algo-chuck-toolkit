use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_trader::TraderClient;

/// Handle the account orders command for data retrieval
pub async fn handle_account_orders_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Account Orders");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required parameters
    let account_number = matches
        .get_one::<String>("account-number")
        .ok_or_else(|| anyhow::anyhow!("Account number is required"))?;

    let from_entered_time = matches
        .get_one::<String>("from-entered-time")
        .ok_or_else(|| anyhow::anyhow!("From entered time is required"))?;

    let to_entered_time = matches
        .get_one::<String>("to-entered-time")
        .ok_or_else(|| anyhow::anyhow!("To entered time is required"))?;

    // Get optional parameters
    let max_results = matches.get_one::<i64>("max-results").copied();
    let status = matches.get_one::<String>("status").map(|s| s.as_str());

    let client = TraderClient::new(reqwest::Client::new());
    let data = client
        .get_orders_by_path_param(
            &access_token,
            account_number,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        )
        .await?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the account order command for data retrieval
pub async fn handle_account_order_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Account Order");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required parameters
    let account_number = matches
        .get_one::<String>("account-number")
        .ok_or_else(|| anyhow::anyhow!("Account number is required"))?;

    let order_id = matches
        .get_one::<i64>("order-id")
        .ok_or_else(|| anyhow::anyhow!("Order ID is required"))?;

    let client = TraderClient::new(reqwest::Client::new());
    let data = client
        .get_order(&access_token, account_number, *order_id)
        .await?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the orders command for data retrieval
pub async fn handle_orders_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Orders");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required parameters
    let from_entered_time = matches
        .get_one::<String>("from-entered-time")
        .ok_or_else(|| anyhow::anyhow!("From entered time is required"))?;

    let to_entered_time = matches
        .get_one::<String>("to-entered-time")
        .ok_or_else(|| anyhow::anyhow!("To entered time is required"))?;

    // Get optional parameters
    let max_results = matches.get_one::<i64>("max-results").copied();
    let status = matches.get_one::<String>("status").map(|s| s.as_str());

    let client = TraderClient::new(reqwest::Client::new());
    let data = client
        .get_orders_by_query_param(
            &access_token,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        )
        .await?;
    println!("{:#?}", data);

    Ok(())
}
