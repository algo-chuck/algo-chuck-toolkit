use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::MarketdataClient;

/// Handle the quotes command for multiple symbols
pub async fn handle_quotes_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Quotes");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required parameters
    let symbols = matches
        .get_one::<String>("symbols")
        .ok_or_else(|| anyhow::anyhow!("Symbols are required"))?;

    // Get optional parameters
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());
    let indicative = matches.get_flag("indicative").then_some(true);

    let client = MarketdataClient::new(reqwest::Client::new());
    let data = client
        .get_quotes(&access_token, symbols, fields, indicative)
        .await?;

    println!("{:#?}", data);

    Ok(())
}

/// Handle the quote command for a single symbol
pub async fn handle_quote_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Quote");

    // Load configuration and TokenManager
    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;

    // Get access token from TokenManager
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    // Get required parameters
    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?;

    // Get optional parameters
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());

    let client = MarketdataClient::new(reqwest::Client::new());
    let data = client.get_quote(&access_token, symbol, fields).await?;

    println!("{:#?}", data);

    Ok(())
}
