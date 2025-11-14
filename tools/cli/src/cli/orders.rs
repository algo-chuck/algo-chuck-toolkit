use clap::{Arg, Command};

/// Build order-related commands
pub fn order_commands() -> Vec<Command> {
    vec![
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
        Command::new("preview-order")
            .about("Preview order for a specific account.")
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
    ]
}
