use clap::{Arg, Command};

/// Build config command and subcommands
pub fn config_command() -> Command {
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
                    Arg::new("auto-refresh")
                        .long("auto-refresh")
                        .value_name("BOOL")
                        .help("Enable/disable automatic token refresh (true/false)"),
                )
                .arg(
                    Arg::new("refresh-buffer")
                        .long("refresh-buffer")
                        .value_name("SECONDS")
                        .help("Set token refresh buffer time in seconds (default: 300)"),
                ),
        )
        .subcommand(Command::new("reset").about("Reset configuration and clear all tokens"))
        .subcommand(Command::new("clean").about("Remove all data and directories completely"))
}
