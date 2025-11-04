use clap::{Arg, Command};

/// Build account-related commands
pub fn account_commands() -> Vec<Command> {
    vec![
        Command::new("account-numbers")
            .about("Get list of account numbers and their encrypted values."),
        Command::new("accounts")
            .about("Get linked account(s) balances and positions for the logged in user.")
            .arg(
                Arg::new("fields")
                    .long("fields")
                    .value_name("FIELDS")
                    .help("Fields to include in response (e.g., 'positions')"),
            ),
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
    ]
}
