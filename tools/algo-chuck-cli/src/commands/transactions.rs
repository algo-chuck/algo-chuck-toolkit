use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_trader::AsyncTraderClient;

/// Handle the transactions command for data retrieval
pub async fn handle_transactions_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Transactions");

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

    let start_date = matches
        .get_one::<String>("start-date")
        .ok_or_else(|| anyhow::anyhow!("Start date is required"))?;

    let end_date = matches
        .get_one::<String>("end-date")
        .ok_or_else(|| anyhow::anyhow!("End date is required"))?;

    let types = matches
        .get_one::<String>("types")
        .ok_or_else(|| anyhow::anyhow!("Types is required"))?;

    // Get optional symbol parameter
    let symbol = matches.get_one::<String>("symbol").map(|s| s.as_str());

    let client = AsyncTraderClient::new(reqwest::Client::new());
    let data = client
        .get_transactions(
            &access_token,
            account_number,
            start_date,
            end_date,
            types,
            symbol,
        )
        .await?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the transactions detail command for data retrieval
pub async fn handle_transaction_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Transaction");

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

    let transaction_id = matches
        .get_one::<i64>("transaction-id")
        .ok_or_else(|| anyhow::anyhow!("Transaction ID is required"))?;

    let client = AsyncTraderClient::new(reqwest::Client::new());
    let data = client
        .get_transaction(&access_token, account_number, *transaction_id)
        .await?;
    println!("{:#?}", data);

    Ok(())
}
