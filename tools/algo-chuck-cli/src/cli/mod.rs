mod accounts;
mod auth;
mod ca;
mod config;
mod instruments;
mod market_hours;
mod movers;
mod option_chains;
mod options_expiration_chain;
mod orders;
mod price_history;
mod quotes;
mod transactions;
mod user;

use clap::Command;

/// Build the complete CLI application structure
pub fn build_cli() -> Command {
    let mut app = Command::new("chuck")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CLI tool for interacting with the Schwab Developer API")
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Add authentication commands (login, refresh, status)
    for cmd in auth::auth_commands() {
        app = app.subcommand(cmd);
    }

    // Add config command
    for cmd in config::config_commands() {
        app = app.subcommand(cmd);
    }

    // Add CA command
    for cmd in ca::ca_commands() {
        app = app.subcommand(cmd);
    }

    // Add account commands
    for cmd in accounts::account_commands() {
        app = app.subcommand(cmd);
    }

    // Add order commands
    for cmd in orders::order_commands() {
        app = app.subcommand(cmd);
    }

    // Add transaction commands
    for cmd in transactions::transaction_commands() {
        app = app.subcommand(cmd);
    }

    // Add user commands
    for cmd in user::user_commands() {
        app = app.subcommand(cmd);
    }

    // Add quote commands
    for cmd in quotes::quote_commands() {
        app = app.subcommand(cmd);
    }

    // Add option chains commands
    for cmd in option_chains::option_chain_commands() {
        app = app.subcommand(cmd);
    }

    // Add options expiration chain commands
    for cmd in options_expiration_chain::options_expiration_chain_commands() {
        app = app.subcommand(cmd);
    }

    // Add price history commands
    for cmd in price_history::price_history_commands() {
        app = app.subcommand(cmd);
    }

    // Add movers commands
    for cmd in movers::movers_commands() {
        app = app.subcommand(cmd);
    }

    // Add market hours commands
    for cmd in market_hours::market_hours_commands() {
        app = app.subcommand(cmd);
    }

    // Add instruments commands
    for cmd in instruments::instruments_commands() {
        app = app.subcommand(cmd);
    }

    app
}
