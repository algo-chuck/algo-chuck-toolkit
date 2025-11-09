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
        // ==================== Configuration ====================
        Some(("config", m)) => handle_config_command(m),

        // ==================== Certificate Authority ====================
        Some(("ca", m)) => tokio::runtime::Runtime::new()?.block_on(handle_ca_command(m)),

        // ==================== Authentication & OAuth ====================
        Some(("login", m)) => tokio::runtime::Runtime::new()?.block_on(handle_login_command(m)),
        Some(("refresh", m)) => tokio::runtime::Runtime::new()?.block_on(handle_refresh_command(m)),
        Some(("status", m)) => tokio::runtime::Runtime::new()?.block_on(handle_status_command(m)),

        // ==================== Trader API - Accounts ====================
        Some(("account-numbers", m)) => handle_account_numbers_command(m),
        Some(("accounts", m)) => handle_accounts_command(m),
        Some(("account", m)) => handle_account_command(m),

        // ==================== Trader API - Orders ====================
        Some(("account-orders", m)) => handle_account_orders_command(m),
        Some(("place-order", m)) => handle_place_order_command(m),
        Some(("account-order", m)) => handle_account_order_command(m),
        Some(("cancel-order", m)) => handle_cancel_order_command(m),
        Some(("replace-order", m)) => handle_replace_order_command(m),
        Some(("orders", m)) => handle_orders_command(m),
        Some(("preview-order", m)) => handle_preview_order_command(m),

        // ==================== Trader API - Transactions ====================
        Some(("transactions", m)) => handle_transactions_command(m),
        Some(("transaction", m)) => handle_transaction_command(m),

        // ==================== Trader API - User Preference ====================
        Some(("user-preference", m)) => handle_user_preference_command(m),

        // ==================== Market Data API - Quotes ====================
        Some(("quotes", m)) => handle_quotes_command(m),
        Some(("quote", m)) => handle_quote_command(m),

        // ==================== Market Data API - Option Chains ====================
        Some(("chain", m)) => handle_chain_command(m),
        Some(("expiration-chain", m)) => handle_expiration_chain_command(m),

        // ==================== Market Data API - Price History ====================
        Some(("price-history", m)) => handle_price_history_command(m),

        // ==================== Market Data API - Movers ====================
        Some(("movers", m)) => handle_movers_command(m),

        // ==================== Market Data API - Market Hours ====================
        Some(("market-hours", m)) => handle_market_hours_command(m),
        Some(("market-hour", m)) => handle_market_hour_command(m),

        // ==================== Market Data API - Instruments ====================
        Some(("instruments", m)) => handle_instruments_command(m),
        Some(("instrument", m)) => handle_instrument_command(m),

        _ => unreachable!("Subcommand is required"),
    }
}
