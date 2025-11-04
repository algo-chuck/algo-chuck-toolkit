mod auth;
mod ca;
mod cli;
mod commands;
mod config;
mod display;
mod oauth;
mod server;

use anyhow::Result;
use commands::{
    handle_account_command, handle_account_numbers_command, handle_account_order_command,
    handle_account_orders_command, handle_accounts_command, handle_ca_command,
    handle_cancel_order_command, handle_config_command, handle_login_command,
    handle_orders_command, handle_place_order_command, handle_preview_order_command,
    handle_refresh_command, handle_replace_order_command, handle_status_command,
    handle_transaction_command, handle_transactions_command, handle_user_preference_command,
};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("login", sub_matches)) => handle_login_command(sub_matches).await,
        Some(("refresh", sub_matches)) => handle_refresh_command(sub_matches).await,
        Some(("status", sub_matches)) => handle_status_command(sub_matches).await,
        Some(("config", sub_matches)) => handle_config_command(sub_matches).await,
        Some(("ca", sub_matches)) => handle_ca_command(sub_matches).await,
        Some(("account-numbers", sub_matches)) => handle_account_numbers_command(sub_matches).await,
        Some(("accounts", sub_matches)) => handle_accounts_command(sub_matches).await,
        Some(("account", sub_matches)) => handle_account_command(sub_matches).await,
        Some(("account-orders", sub_matches)) => handle_account_orders_command(sub_matches).await,
        Some(("account-order", sub_matches)) => handle_account_order_command(sub_matches).await,
        Some(("orders", sub_matches)) => handle_orders_command(sub_matches).await,
        Some(("place-order", sub_matches)) => handle_place_order_command(sub_matches).await,
        Some(("cancel-order", sub_matches)) => handle_cancel_order_command(sub_matches).await,
        Some(("replace-order", sub_matches)) => handle_replace_order_command(sub_matches).await,
        Some(("preview-order", sub_matches)) => handle_preview_order_command(sub_matches).await,
        Some(("transactions", sub_matches)) => handle_transactions_command(sub_matches).await,
        Some(("transaction", sub_matches)) => handle_transaction_command(sub_matches).await,
        Some(("user-preference", sub_matches)) => handle_user_preference_command(sub_matches).await,
        _ => unreachable!("Subcommand is required"),
    }
}
