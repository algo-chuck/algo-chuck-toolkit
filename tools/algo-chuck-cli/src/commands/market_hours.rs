use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::SyncMarketdataClient;

/// Handle the market hours command
pub fn handle_market_hours_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Market Hours");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let markets = matches
        .get_one::<String>("markets")
        .ok_or_else(|| anyhow::anyhow!("Markets are required"))?;

    let date = matches.get_one::<String>("date").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let data = client.get_market_hours( markets, date)?;

    println!("{:#?}", data);
    Ok(())
}

/// Handle the market hours command
pub fn handle_market_hour_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Market Hour");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let market = matches
        .get_one::<String>("market")
        .ok_or_else(|| anyhow::anyhow!("Market is required"))?;

    let date = matches.get_one::<String>("date").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let data = client.get_market_hour( market, date)?;

    println!("{:#?}", data);
    Ok(())
}
