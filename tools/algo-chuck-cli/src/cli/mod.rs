mod accounts;
mod auth;
mod ca;
mod config;
mod orders;
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
    app = app.subcommand(config::config_command());

    // Add CA command
    app = app.subcommand(ca::ca_command());

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

    app
}
