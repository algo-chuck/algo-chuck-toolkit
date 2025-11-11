use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::SyncMarketdataClient;

/// Handle the instruments command
pub fn handle_instruments_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Instruments");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?;

    let projection = matches
        .get_one::<String>("projection")
        .ok_or_else(|| anyhow::anyhow!("Projection is required"))?;

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let data = client
        .get_instruments( symbol, projection)
        ?;

    println!("{:#?}", data);
    Ok(())
}

/// Handle the instrument command (by CUSIP)
pub fn handle_instrument_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Instrument");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let cusip = matches
        .get_one::<String>("cusip")
        .ok_or_else(|| anyhow::anyhow!("CUSIP is required"))?;

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let data = client
        .get_instruments_by_cusip( cusip)
        ?;

    println!("{:#?}", data);
    Ok(())
}
