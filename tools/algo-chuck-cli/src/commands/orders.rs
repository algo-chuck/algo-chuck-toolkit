use anyhow::Result;
use clap::ArgMatches;
use serde::de::DeserializeOwned;
use std::io::Read;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_trader::SyncTraderClient;

/// Handle the account orders command for data retrieval
pub fn handle_account_orders_command(matches: &ArgMatches) -> Result<()> {
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
    let max_results = matches.get_one::<i64>("max-results").map(|x| *x as i32);
    let status = matches.get_one::<String>("status").map(|s| s.as_str());

    let client = SyncTraderClient::new(ureq::Agent::new());
    let data = client
        .get_orders_by_path_param(
            &access_token,
            account_number,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        )
        ?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the account order command for data retrieval
pub fn handle_account_order_command(matches: &ArgMatches) -> Result<()> {
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

    let client = SyncTraderClient::new(ureq::Agent::new());
    let data = client
        .get_order(&access_token, account_number, *order_id)
        ?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the orders command for data retrieval
pub fn handle_orders_command(matches: &ArgMatches) -> Result<()> {
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
    let max_results = matches.get_one::<i64>("max-results").map(|x| *x as i32);
    let status = matches.get_one::<String>("status").map(|s| s.as_str());

    let client = SyncTraderClient::new(ureq::Agent::new());
    let data = client
        .get_orders_by_query_param(
            &access_token,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        )
        ?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the place order command
pub fn handle_place_order_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Placing Order");

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

    // Read order JSON from file or stdin
    let order_json = read_json(matches)?;

    let client = SyncTraderClient::new(ureq::Agent::new());
    client
        .place_order(&access_token, account_number, &order_json)
        ?;

    println!("✅ Order placed successfully");

    Ok(())
}

/// Handle the cancel order command
pub fn handle_cancel_order_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Canceling Order");

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

    let client = SyncTraderClient::new(ureq::Agent::new());
    client
        .cancel_order(&access_token, account_number, *order_id)
        ?;

    println!("✅ Order canceled successfully");

    Ok(())
}

/// Handle the replace order command
pub fn handle_replace_order_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Replacing Order");

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

    // Read order JSON from file or stdin
    let order_json = read_json(matches)?;

    let client = SyncTraderClient::new(ureq::Agent::new());
    client
        .replace_order(&access_token, account_number, *order_id, &order_json)
        ?;

    println!("✅ Order replaced successfully");

    Ok(())
}

/// Handle the preview order command
pub fn handle_preview_order_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Previewing Order");

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

    // Read preview JSON from file or stdin
    let preview_json = read_json(matches)?;

    let client = SyncTraderClient::new(ureq::Agent::new());
    let preview = client
        .preview_order(&access_token, account_number, &preview_json)
        ?;

    println!("{:#?}", preview);

    Ok(())
}

/// Generic helper function to read and parse JSON from file or stdin
fn read_json<T: DeserializeOwned>(matches: &ArgMatches) -> Result<T> {
    let json_str = if let Some(file_path) = matches.get_one::<String>("order-file") {
        // Read from file
        std::fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file_path, e))?
    } else if matches.get_flag("stdin") {
        // Read from stdin
        let mut buffer = String::new();
        Read::read_to_string(&mut std::io::stdin(), &mut buffer)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {}", e))?;
        buffer
    } else {
        return Err(anyhow::anyhow!(
            "JSON must be provided via --order-file or --stdin"
        ));
    };

    // Parse JSON string into type T
    serde_json::from_str(&json_str).map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
}
