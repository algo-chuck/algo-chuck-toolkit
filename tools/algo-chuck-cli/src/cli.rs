use clap::{Arg, Command};

/// Build the complete CLI application structure
pub fn build_cli() -> Command {
    Command::new("chuck")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CLI tool for interacting with the Schwab Developer API")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("login").about("Perform initial OAuth2 authentication"))
        .subcommand(Command::new("refresh").about("Refresh access token using refresh token"))
        .subcommand(Command::new("info").about("Display current token status and expiry times"))
        .subcommand(
            Command::new("config")
                .about("Manage configuration settings")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("show").about("Display current configuration"))
                .subcommand(
                    Command::new("set")
                        .about("Set configuration values")
                        .arg(
                            Arg::new("client-id")
                                .long("client-id")
                                .value_name("CLIENT_ID")
                                .help("Set Schwab API client ID"),
                        )
                        .arg(
                            Arg::new("client-secret")
                                .long("client-secret")
                                .value_name("CLIENT_SECRET")
                                .help("Set Schwab API client secret"),
                        )
                        .arg(
                            Arg::new("callback-url")
                                .long("callback-url")
                                .value_name("URL")
                                .help("Set OAuth2 callback URL"),
                        )
                        .arg(
                            Arg::new("auto-refresh")
                                .long("auto-refresh")
                                .value_name("BOOL")
                                .help("Enable/disable automatic token refresh (true/false)"),
                        ),
                )
                .subcommand(Command::new("reset").about("Reset configuration and clear all tokens"))
                .subcommand(
                    Command::new("clean").about("Remove all data and directories completely"),
                ),
        )
}
