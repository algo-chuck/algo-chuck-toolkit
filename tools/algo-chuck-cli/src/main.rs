mod auth;
mod ca;
mod cli;
mod commands;
mod config;
mod display;
mod oauth;
mod server;

use anyhow::Result;
use commands::*;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        // Commands that need async runtime (OAuth + server)
        Some(("login", m)) => tokio::runtime::Runtime::new()?.block_on(handle_login_command(m)),
        Some(("refresh", m)) => tokio::runtime::Runtime::new()?.block_on(handle_refresh_command(m)),

        // Synchronous commands - no runtime overhead!
        Some(("account-numbers", m)) => handle_account_numbers_command(m),
        Some(("market-hours", m)) => handle_market_hours_command(m),
        Some(("market-hour", m)) => handle_market_hour_command(m),

        // Rest of the commands still async for now (will convert later)
        Some(("status", m)) => tokio::runtime::Runtime::new()?.block_on(handle_status_command(m)),
        Some(("config", m)) => tokio::runtime::Runtime::new()?.block_on(handle_config_command(m)),
        Some(("ca", m)) => tokio::runtime::Runtime::new()?.block_on(handle_ca_command(m)),
        Some(("accounts", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_accounts_command(m))
        }
        Some(("account", m)) => tokio::runtime::Runtime::new()?.block_on(handle_account_command(m)),
        Some(("account-orders", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_account_orders_command(m))
        }
        Some(("account-order", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_account_order_command(m))
        }
        Some(("orders", m)) => tokio::runtime::Runtime::new()?.block_on(handle_orders_command(m)),
        Some(("place-order", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_place_order_command(m))
        }
        Some(("cancel-order", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_cancel_order_command(m))
        }
        Some(("replace-order", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_replace_order_command(m))
        }
        Some(("preview-order", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_preview_order_command(m))
        }
        Some(("transactions", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_transactions_command(m))
        }
        Some(("transaction", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_transaction_command(m))
        }
        Some(("user-preference", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_user_preference_command(m))
        }
        Some(("quotes", m)) => tokio::runtime::Runtime::new()?.block_on(handle_quotes_command(m)),
        Some(("quote", m)) => tokio::runtime::Runtime::new()?.block_on(handle_quote_command(m)),
        Some(("chain", m)) => tokio::runtime::Runtime::new()?.block_on(handle_chain_command(m)),
        Some(("expiration-chain", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_expiration_chain_command(m))
        }
        Some(("price-history", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_price_history_command(m))
        }
        Some(("movers", m)) => tokio::runtime::Runtime::new()?.block_on(handle_movers_command(m)),
        Some(("instruments", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_instruments_command(m))
        }
        Some(("instrument", m)) => {
            tokio::runtime::Runtime::new()?.block_on(handle_instrument_command(m))
        }
        _ => unreachable!("Subcommand is required"),
    }
}
