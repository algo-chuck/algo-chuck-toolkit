use anyhow::Result;
use clap::ArgMatches;
use schwab_api::prelude::{SyncTraderClient, trader};

use crate::config::{ConfigManager, TokenManager};

/// Handle the transactions command for data retrieval
pub fn handle_transactions_command(matches: &ArgMatches) -> Result<()> {
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
        .ok_or_else(|| anyhow::anyhow!("Account number is required"))?
        .as_str();

    let start_date = matches
        .get_one::<String>("start-date")
        .ok_or_else(|| anyhow::anyhow!("Start date is required"))?
        .as_str();

    let end_date = matches
        .get_one::<String>("end-date")
        .ok_or_else(|| anyhow::anyhow!("End date is required"))?
        .as_str();

    let types = matches
        .get_one::<String>("types")
        .ok_or_else(|| anyhow::anyhow!("Types is required"))?
        .as_str();

    // Get optional symbol parameter
    let symbol = matches.get_one::<String>("symbol").map(|s| s.as_str());

    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let params = trader::GetTransactionsByPathParams {
        account_hash: account_number,
        start_date,
        end_date,
        types,
        symbol,
    };
    let data = client.get_transactions_by_path_param(&params)?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the transactions detail command for data retrieval
pub fn handle_transaction_command(matches: &ArgMatches) -> Result<()> {
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
        .ok_or_else(|| anyhow::anyhow!("Account number is required"))?
        .as_str();

    let transaction_id = matches
        .get_one::<i64>("transaction-id")
        .ok_or_else(|| anyhow::anyhow!("Transaction ID is required"))?;

    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let params = trader::GetTransactionByIdParams {
        account_hash: account_number,
        transaction_id: *transaction_id,
    };
    let data = client.get_transactions_by_id(&params)?;
    println!("{:#?}", data);

    Ok(())
}
