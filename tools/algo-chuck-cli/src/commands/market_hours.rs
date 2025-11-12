use anyhow::Result;
use clap::ArgMatches;
use schwab_api::prelude::{SyncMarketdataClient, marketdata};

use crate::config::{ConfigManager, TokenManager};

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
        .ok_or_else(|| anyhow::anyhow!("Markets are required"))?
        .as_str();

    let date = matches.get_one::<String>("date").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = marketdata::GetMarketHoursParams { markets, date };
    let data = client.get_market_hours(&params)?;

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
        .ok_or_else(|| anyhow::anyhow!("Market is required"))?
        .as_str();

    let date = matches.get_one::<String>("date").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = marketdata::GetMarketHourParams { market, date };
    let data = client.get_market_hour(&params)?;

    println!("{:#?}", data);
    Ok(())
}
