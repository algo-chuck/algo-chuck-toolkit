use anyhow::Result;
use clap::ArgMatches;

use super::accounts::{
    handle_account_command, handle_account_numbers_command, handle_accounts_command,
};
use super::transactions::{handle_transaction_command, handle_transactions_command};
use super::user_preference::handle_user_preference_command;

/// Handle the trader command for data retrieval
pub async fn handle_trader_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("account-numbers", sub_matches)) => {
            handle_account_numbers_command(sub_matches).await?
        }
        Some(("accounts", sub_matches)) => handle_accounts_command(sub_matches).await?,
        Some(("account", sub_matches)) => handle_account_command(sub_matches).await?,
        Some(("transactions", sub_matches)) => handle_transactions_command(sub_matches).await?,
        Some(("transaction", sub_matches)) => handle_transaction_command(sub_matches).await?,
        Some(("user-preference", sub_matches)) => {
            handle_user_preference_command(sub_matches).await?
        }
        _ => unreachable!("Trader subcommand is required"),
    }

    Ok(())
}
