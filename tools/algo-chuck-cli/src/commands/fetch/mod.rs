use anyhow::Result;
use clap::ArgMatches;

use trader::handle_trader_command;

mod trader;
pub mod user_preference;

/// Handle the fetch command for data retrieval
pub async fn handle_fetch_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("trader", sub_matches)) => handle_trader_command(sub_matches).await?,
        _ => unreachable!("Fetch subcommand is required"),
    }

    Ok(())
}
