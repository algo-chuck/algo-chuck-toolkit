use clap::{Arg, ArgAction, Command};

pub fn price_history_commands() -> Vec<Command> {
    vec![
        Command::new("price-history")
            .about("Get price history for a symbol")
            .arg(
                Arg::new("symbol")
                    .help("Symbol (e.g., AAPL)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("period-type")
                    .long("period-type")
                    .help("Period type: day, month, year, ytd")
                    .value_name("TYPE"),
            )
            .arg(
                Arg::new("period")
                    .long("period")
                    .help("Number of periods")
                    .value_name("NUM"),
            )
            .arg(
                Arg::new("frequency-type")
                    .long("frequency-type")
                    .help("Frequency type: minute, daily, weekly, monthly")
                    .value_name("TYPE"),
            )
            .arg(
                Arg::new("frequency")
                    .long("frequency")
                    .help("Frequency: 1, 5, 10, 15, 30 (minute); 1 (daily/weekly/monthly)")
                    .value_name("NUM"),
            )
            .arg(
                Arg::new("start-date")
                    .long("start-date")
                    .help("Start date in milliseconds since epoch")
                    .value_name("MILLIS"),
            )
            .arg(
                Arg::new("end-date")
                    .long("end-date")
                    .help("End date in milliseconds since epoch")
                    .value_name("MILLIS"),
            )
            .arg(
                Arg::new("extended-hours")
                    .long("extended-hours")
                    .help("Include extended hours data")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("previous-close")
                    .long("previous-close")
                    .help("Include previous close")
                    .action(ArgAction::SetTrue),
            ),
    ]
}
