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
    handle_cancel_order_command, handle_config_command, handle_expiration_chain_command,
    handle_instrument_command, handle_instruments_command, handle_login_command,
    handle_market_command, handle_markets_command, handle_movers_command,
    handle_option_chain_command, handle_orders_command, handle_place_order_command,
    handle_preview_order_command, handle_price_history_command, handle_quote_command,
    handle_quotes_command, handle_refresh_command, handle_replace_order_command,
    handle_status_command, handle_transaction_command, handle_transactions_command,
    handle_user_preference_command,
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
        Some(("quotes", sub_matches)) => handle_quotes_command(sub_matches).await,
        Some(("quote", sub_matches)) => handle_quote_command(sub_matches).await,
        Some(("option-chain", sub_matches)) => handle_option_chain_command(sub_matches).await,
        Some(("expiration-chain", sub_matches)) => {
            handle_expiration_chain_command(sub_matches).await
        }
        Some(("price-history", sub_matches)) => handle_price_history_command(sub_matches).await,
        Some(("movers", sub_matches)) => handle_movers_command(sub_matches).await,
        Some(("markets", sub_matches)) => handle_markets_command(sub_matches).await,
        Some(("market", sub_matches)) => handle_market_command(sub_matches).await,
        Some(("instruments", sub_matches)) => handle_instruments_command(sub_matches).await,
        Some(("instrument", sub_matches)) => handle_instrument_command(sub_matches).await,
        _ => unreachable!("Subcommand is required"),
    }
}
