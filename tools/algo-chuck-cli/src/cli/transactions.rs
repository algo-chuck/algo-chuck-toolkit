use clap::{Arg, Command};

/// Build transaction-related commands
pub fn transaction_commands() -> Vec<Command> {
    vec![
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
    ]
}
