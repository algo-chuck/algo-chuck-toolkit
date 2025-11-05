use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::MarketdataClient;

/// Handle the markets command
pub async fn handle_markets_command(matches: &ArgMatches) -> Result<()> {
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

    let client = MarketdataClient::new(reqwest::Client::new());
    let data = client.get_markets(&access_token, markets, date).await?;

    println!("{:#?}", data);
    Ok(())
}

/// Handle the market command
pub async fn handle_market_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Market Hours");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let market = matches
        .get_one::<String>("market")
        .ok_or_else(|| anyhow::anyhow!("Market is required"))?;

    let date = matches.get_one::<String>("date").map(|s| s.as_str());

    let client = MarketdataClient::new(reqwest::Client::new());
    let data = client.get_market(&access_token, market, date).await?;

    println!("{:#?}", data);
    Ok(())
}
