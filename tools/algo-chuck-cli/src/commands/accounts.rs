use anyhow::Result;
use clap::ArgMatches;
use schwab_api::prelude::{SyncTraderClient, types::trader};

use crate::config::{ConfigManager, TokenManager};

/// Handle the account numbers command for data retrieval (synchronous)
pub fn handle_account_numbers_command(_matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Account Numbers");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Create client with access token
    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let data = client.get_account_numbers()?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the accounts command for data retrieval
pub fn handle_accounts_command(matches: &ArgMatches) -> Result<()> {
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

    // Create client with access token
    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let params = trader::GetAccountsParams { fields };
    let data = client.get_accounts(&params)?;
    println!("{:#?}", data);

    Ok(())
}

/// Handle the account command for data retrieval
pub fn handle_account_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Single Account");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required account-number argument
    let account_number = matches
        .get_one::<String>("account-number")
        .ok_or_else(|| anyhow::anyhow!("account-number is required"))?
        .as_str();

    // Get optional fields parameter
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());

    // Create client with access token
    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let params = trader::GetAccountParams {
        account_hash: account_number,
        fields,
    };
    let data = client.get_account(&params)?;
    println!("{:#?}", data);

    Ok(())
}
