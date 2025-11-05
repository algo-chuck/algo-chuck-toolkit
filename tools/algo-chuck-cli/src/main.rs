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
    handle_cancel_order_command, handle_chain_command, handle_config_command,
    handle_expiration_chain_command, handle_instrument_command, handle_instruments_command,
    handle_login_command, handle_market_hour_command, handle_market_hours_command,
    handle_movers_command, handle_orders_command, handle_place_order_command,
    handle_preview_order_command, handle_price_history_command, handle_quote_command,
    handle_quotes_command, handle_refresh_command, handle_replace_order_command,
    handle_status_command, handle_transaction_command, handle_transactions_command,
    handle_user_preference_command,
};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("login", m)) => handle_login_command(m).await,
        Some(("refresh", m)) => handle_refresh_command(m).await,
        Some(("status", m)) => handle_status_command(m).await,
        Some(("config", m)) => handle_config_command(m).await,
        Some(("ca", m)) => handle_ca_command(m).await,
        Some(("account-numbers", m)) => handle_account_numbers_command(m).await,
        Some(("accounts", m)) => handle_accounts_command(m).await,
        Some(("account", m)) => handle_account_command(m).await,
        Some(("account-orders", m)) => handle_account_orders_command(m).await,
        Some(("account-order", m)) => handle_account_order_command(m).await,
        Some(("orders", m)) => handle_orders_command(m).await,
        Some(("place-order", m)) => handle_place_order_command(m).await,
        Some(("cancel-order", m)) => handle_cancel_order_command(m).await,
        Some(("replace-order", m)) => handle_replace_order_command(m).await,
        Some(("preview-order", m)) => handle_preview_order_command(m).await,
        Some(("transactions", m)) => handle_transactions_command(m).await,
        Some(("transaction", m)) => handle_transaction_command(m).await,
        Some(("user-preference", m)) => handle_user_preference_command(m).await,
        Some(("quotes", m)) => handle_quotes_command(m).await,
        Some(("quote", m)) => handle_quote_command(m).await,
        Some(("chain", m)) => handle_chain_command(m).await,
        Some(("expiration-chain", m)) => handle_expiration_chain_command(m).await,
        Some(("price-history", m)) => handle_price_history_command(m).await,
        Some(("movers", m)) => handle_movers_command(m).await,
        Some(("market-hours", m)) => handle_market_hours_command(m).await,
        Some(("market-hour", m)) => handle_market_hour_command(m).await,
        Some(("instruments", m)) => handle_instruments_command(m).await,
        Some(("instrument", m)) => handle_instrument_command(m).await,
        _ => unreachable!("Subcommand is required"),
    }
}
