use anyhow::Result;
use clap::ArgMatches;
use schwab_api::prelude::{SyncMarketdataClient, types::marketdata};

use crate::config::{ConfigManager, TokenManager};

/// Handle the expiration-chain command
pub fn handle_expiration_chain_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Expiration Chain");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?
        .as_str();

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = marketdata::GetExpirationChainParams { symbol };
    let data = client.get_expiration_chain(&params)?;

    println!("{:#?}", data);
    Ok(())
}
