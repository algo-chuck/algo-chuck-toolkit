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
                .subcommand(
                    Command::new("clean").about("Remove all data and directories completely"),
                ),
        )
        .subcommand(
            Command::new("ca")
                .about("Manage Certificate Authority for seamless HTTPS")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("status").about("Show CA certificate status"))
                .subcommand(
                    Command::new("install").about("Install CA certificate in system trust store"),
                )
                .subcommand(
                    Command::new("uninstall")
                        .about("Remove CA certificate from system trust store"),
                )
                .subcommand(Command::new("regenerate").about("Regenerate CA certificate"))
                .subcommand(
                    Command::new("show").about("Display CA certificate for manual installation"),
                )
                .subcommand(
                    Command::new("clean")
                        .about("Remove all CA certificates and optionally uninstall from system")
                        .arg(
                            Arg::new("uninstall")
                                .long("uninstall")
                                .help("Also remove CA from system trust store")
                                .action(clap::ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("test")
                        .about("Test certificate by opening browser to HTTPS server")
                        .arg(
                            Arg::new("port")
                                .long("port")
                                .help("Port to run the test server on")
                                .default_value("8443")
                                .value_parser(clap::value_parser!(u16)),
                        ),
                ),
        )
}
