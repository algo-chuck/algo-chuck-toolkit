use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_trader::SyncTraderClient;

/// Handle the user preference command for data retrieval
pub fn handle_user_preference_command(_matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Logged In User's Preference");
    // Implement the logic to fetch user preferences here

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let client = SyncTraderClient::new(ureq::Agent::new(), access_token);
    let data = client.get_user_preference()?;
    println!("{:#?}", data);

    Ok(())
}
