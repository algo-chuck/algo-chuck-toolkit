use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::MarketdataClient;

/// Handle the expiration-chain command
pub async fn handle_expiration_chain_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Expiration Chain");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?;

    let client = MarketdataClient::new(reqwest::Client::new());
    let data = client.get_expiration_chain(&access_token, symbol).await?;

    println!("{:#?}", data);
    Ok(())
}
