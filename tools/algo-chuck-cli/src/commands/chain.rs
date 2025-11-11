use anyhow::Result;
use clap::ArgMatches;

use crate::config::{ConfigManager, TokenManager};
use schwab_api_marketdata::SyncMarketdataClient;
use schwab_api_types::marketdata_params::GetChainParams;

/// Handle the option-chain command
pub fn handle_chain_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Fetching Option Chain");

    let config_manager = ConfigManager::new()?;
    let token_manager = TokenManager::new(&config_manager)?;
    let access_token = token_manager
        .get_access_token()?
        .ok_or_else(|| anyhow::anyhow!("No access token found. Please run 'chuck login' first."))?;

    let symbol = matches
        .get_one::<String>("symbol")
        .ok_or_else(|| anyhow::anyhow!("Symbol is required"))?
        .as_str();

    let contract_type = matches
        .get_one::<String>("contract-type")
        .map(|s| s.as_str());
    let strike_count = matches
        .get_one::<String>("strike-count")
        .and_then(|s| s.parse().ok());
    let include_underlying_quote = matches.get_flag("include-quote").then_some(true);
    let strategy = matches.get_one::<String>("strategy").map(|s| s.as_str());
    let interval = matches
        .get_one::<String>("interval")
        .and_then(|s| s.parse().ok());
    let strike = matches
        .get_one::<String>("strike")
        .and_then(|s| s.parse().ok());
    let range = matches.get_one::<String>("range").map(|s| s.as_str());
    let from_date = matches.get_one::<String>("from-date").map(|s| s.as_str());
    let to_date = matches.get_one::<String>("to-date").map(|s| s.as_str());
    let volatility = matches
        .get_one::<String>("volatility")
        .and_then(|s| s.parse().ok());
    let underlying_price = matches
        .get_one::<String>("underlying-price")
        .and_then(|s| s.parse().ok());
    let interest_rate = matches
        .get_one::<String>("interest-rate")
        .and_then(|s| s.parse().ok());
    let days_to_expiration = matches
        .get_one::<String>("days-to-expiration")
        .and_then(|s| s.parse().ok());
    let exp_month = matches.get_one::<String>("exp-month").map(|s| s.as_str());
    let option_type = matches.get_one::<String>("option-type").map(|s| s.as_str());

    let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
    let params = GetChainParams {
        symbol,
        contract_type,
        strike_count,
        include_underlying_quote,
        strategy,
        interval,
        strike,
        range,
        from_date,
        to_date,
        volatility,
        underlying_price,
        interest_rate,
        days_to_expiration,
        exp_month,
        option_type,
    };
    let data = client.get_chain(&params)?;

    println!("{:#?}", data);
    Ok(())
}
