use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::SyncMarketdataClient;
use schwab_api_types::marketdata::{GetQuoteParams, GetQuotesParams};

/// Handle the quotes command for multiple symbols
pub fn handle_quotes_command(matches: &ArgMatches) -> Result<()> {
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

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = GetQuotesParams {
        symbols: symbols.as_str(),
        fields,
        indicative,
    };
    let data = client.get_quotes(&params)?;

    println!("{:#?}", data);

    Ok(())
}

/// Handle the quote command for a single symbol
pub fn handle_quote_command(matches: &ArgMatches) -> Result<()> {
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
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?
        .as_str();

    // Get optional parameters
    let fields = matches.get_one::<String>("fields").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = GetQuoteParams { symbol, fields };
    let data = client.get_quote(&params)?;

    println!("{:#?}", data);
    Ok(())
}
