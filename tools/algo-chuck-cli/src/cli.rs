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
        .subcommand(Command::new("status").about("Display current token status and expiry times"))
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
        .subcommand(
            Command::new("account-numbers")
                .about("Get list of account numbers and their encrypted values."),
        )
        .subcommand(
            Command::new("accounts")
                .about("Get linked account(s) balances and positions for the logged in user.")
                .arg(
                    Arg::new("fields")
                        .long("fields")
                        .value_name("FIELDS")
                        .help("Fields to include in response (e.g., 'positions')"),
                ),
        )
        .subcommand(
            Command::new("account")
                .about("Get a specific account balance and positions for the logged in user.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("fields")
                        .long("fields")
                        .value_name("FIELDS")
                        .help("Fields to include in response (e.g., 'positions')"),
                ),
        )
        .subcommand(
            Command::new("account-orders")
                .about("Get all orders for a specific account.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("from-entered-time")
                        .long("from-entered-time")
                        .value_name("FROM_TIME")
                        .help("Start time in ISO-8601 format (e.g., '2024-03-29T00:00:00.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("to-entered-time")
                        .long("to-entered-time")
                        .value_name("TO_TIME")
                        .help("End time in ISO-8601 format (e.g., '2024-04-28T23:59:59.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("max-results")
                        .long("max-results")
                        .value_name("MAX_RESULTS")
                        .help("Maximum number of orders to retrieve (default: 3000)")
                        .value_parser(clap::value_parser!(i64)),
                )
                .arg(
                    Arg::new("status")
                        .long("status")
                        .value_name("STATUS")
                        .help("Filter by order status"),
                ),
        )
        .subcommand(
            Command::new("account-order")
                .about("Get a specific order by its ID, for a specific account.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("order-id")
                        .long("order-id")
                        .short('o')
                        .value_name("ORDER_ID")
                        .help("The ID of the order")
                        .required(true)
                        .value_parser(clap::value_parser!(i64)),
                ),
        )
        .subcommand(
            Command::new("orders")
                .about("Get all orders for all accounts.")
                .arg(
                    Arg::new("from-entered-time")
                        .long("from-entered-time")
                        .value_name("FROM_TIME")
                        .help("Start time in ISO-8601 format (e.g., '2024-03-29T00:00:00.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("to-entered-time")
                        .long("to-entered-time")
                        .value_name("TO_TIME")
                        .help("End time in ISO-8601 format (e.g., '2024-04-28T23:59:59.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("max-results")
                        .long("max-results")
                        .value_name("MAX_RESULTS")
                        .help("Maximum number of orders to retrieve (default: 3000)")
                        .value_parser(clap::value_parser!(i64)),
                )
                .arg(
                    Arg::new("status")
                        .long("status")
                        .value_name("STATUS")
                        .help("Filter by order status"),
                ),
        )
        .subcommand(
            Command::new("place-order")
                .about("Place order for a specific account.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("order-file")
                        .long("order-file")
                        .short('f')
                        .value_name("FILE")
                        .help("Path to JSON file containing order request")
                        .conflicts_with("stdin"),
                )
                .arg(
                    Arg::new("stdin")
                        .long("stdin")
                        .help("Read order JSON from stdin")
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("order-file"),
                ),
        )
        .subcommand(
            Command::new("cancel-order")
                .about("Cancel an order for a specific account")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("order-id")
                        .long("order-id")
                        .short('o')
                        .value_name("ORDER_ID")
                        .help("The ID of the order to cancel")
                        .required(true)
                        .value_parser(clap::value_parser!(i64)),
                ),
        )
        .subcommand(
            Command::new("replace-order")
                .about("Replace order for a specific account")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("order-id")
                        .long("order-id")
                        .short('o')
                        .value_name("ORDER_ID")
                        .help("The ID of the order to replace")
                        .required(true)
                        .value_parser(clap::value_parser!(i64)),
                )
                .arg(
                    Arg::new("order-file")
                        .long("order-file")
                        .short('f')
                        .value_name("FILE")
                        .help("Path to JSON file containing replacement order request")
                        .conflicts_with("stdin"),
                )
                .arg(
                    Arg::new("stdin")
                        .long("stdin")
                        .help("Read order JSON from stdin")
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("order-file"),
                ),
        )
        .subcommand(
            Command::new("preview-order")
                .about("Preview order for a specific account. **Coming Soon**.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("order-file")
                        .long("order-file")
                        .short('f')
                        .value_name("FILE")
                        .help("Path to JSON file containing order request")
                        .conflicts_with("stdin"),
                )
                .arg(
                    Arg::new("stdin")
                        .long("stdin")
                        .help("Read order JSON from stdin")
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("order-file"),
                ),
        )
        .subcommand(
            Command::new("transactions")
                .about("Get all transactions information for a specific account.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("start-date")
                        .long("start-date")
                        .value_name("START_DATE")
                        .help("Start date in ISO-8601 format (e.g., '2024-03-28T21:10:42.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("end-date")
                        .long("end-date")
                        .value_name("END_DATE")
                        .help("End date in ISO-8601 format (e.g., '2024-05-10T21:10:42.000Z')")
                        .required(true),
                )
                .arg(
                    Arg::new("types")
                        .long("types")
                        .value_name("TYPES")
                        .help("Transaction types to filter")
                        .required(true),
                )
                .arg(
                    Arg::new("symbol")
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Filter by symbol (optional)"),
                ),
        )
        .subcommand(
            Command::new("transaction")
                .about("Get specific transaction information for a specific account.")
                .arg(
                    Arg::new("account-number")
                        .long("account-number")
                        .short('a')
                        .value_name("ACCOUNT_NUMBER")
                        .help("The encrypted ID of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("transaction-id")
                        .long("transaction-id")
                        .short('t')
                        .value_name("TRANSACTION_ID")
                        .help("The ID of the transaction")
                        .required(true)
                        .value_parser(clap::value_parser!(i64)),
                ),
        )
        .subcommand(
            Command::new("user-preference")
                .about("Get user preference information for the logged in user."),
        )
}
