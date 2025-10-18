use anyhow::Result;
use clap::ArgMatches;

use crate::commands::fetch::user_preference::handle_user_preference_command;

/// Handle the trader command for data retrieval
pub async fn handle_trader_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("user-preference", sub_matches)) => {
            handle_user_preference_command(sub_matches).await?
        }
        _ => unreachable!("Trader subcommand is required"),
    }

    Ok(())
}
