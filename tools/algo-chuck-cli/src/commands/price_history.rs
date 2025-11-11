use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::SyncMarketdataClient;

/// Handle the price-history command
pub fn handle_price_history_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Price History");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?;

    let period_type = matches.get_one::<String>("period-type").map(|s| s.as_str());
    let period = matches
        .get_one::<String>("period")
        .and_then(|s| s.parse().ok());
    let frequency_type = matches
        .get_one::<String>("frequency-type")
        .map(|s| s.as_str());
    let frequency = matches
        .get_one::<String>("frequency")
        .and_then(|s| s.parse().ok());
    let start_date = matches
        .get_one::<String>("start-date")
        .and_then(|s| s.parse().ok());
    let end_date = matches
        .get_one::<String>("end-date")
        .and_then(|s| s.parse().ok());
    let need_extended_hours = matches.get_flag("extended-hours").then_some(true);
    let need_previous_close = matches.get_flag("previous-close").then_some(true);

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let data = client.get_price_history(
        symbol,
        period_type,
        period,
        frequency_type,
        frequency,
        start_date,
        end_date,
        need_extended_hours,
        need_previous_close,
    )?;

    println!("{:#?}", data);
    Ok(())
}
