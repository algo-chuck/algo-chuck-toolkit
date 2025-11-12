use anyhow::Result;
use clap::ArgMatches;
use schwab_api::prelude::{SyncMarketdataClient, types::marketdata};

use crate::config::{ConfigManager, TokenManager};

/// Handle the movers command
pub fn handle_movers_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Movers");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?
        .as_str();

    let sort = matches.get_one::<String>("sort").map(|s| s.as_str());
    let frequency = matches
        .get_one::<String>("frequency")
        .and_then(|s| s.parse().ok());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = marketdata::GetMoversParams {
        symbol,
        sort,
        frequency,
    };
    let data = client.get_movers(&params)?;

    println!("{:#?}", data);
    Ok(())
}
