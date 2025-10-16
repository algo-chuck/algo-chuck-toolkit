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
    handle_ca_command, handle_config_command, handle_login_command, handle_refresh_command,
    handle_status_command,
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
        _ => unreachable!("Subcommand is required"),
    }
}
