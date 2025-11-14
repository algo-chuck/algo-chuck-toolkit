use clap::{Arg, ArgAction, Command};

pub fn chain_commands() -> Vec<Command> {
    vec![
        Command::new("chain")
            .about("Get option chain for an optionable symbol")
            .arg(
                Arg::new("symbol")
                    .help("Symbol (e.g., AAPL)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("contract-type")
                    .long("contract-type")
                    .help("Contract type: CALL, PUT, ALL")
                    .value_name("TYPE"),
            )
            .arg(
                Arg::new("strike-count")
                    .long("strike-count")
                    .help("Number of strikes above/below ATM")
                    .value_name("COUNT"),
            )
            .arg(
                Arg::new("include-quote")
                    .long("include-quote")
                    .help("Include underlying quote")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("strategy")
                    .long("strategy")
                    .help("Strategy: SINGLE, ANALYTICAL, COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, etc.")
                    .value_name("STRATEGY"),
            )
            .arg(
                Arg::new("interval")
                    .long("interval")
                    .help("Strike interval for spread strategies")
                    .value_name("INTERVAL"),
            )
            .arg(
                Arg::new("strike")
                    .long("strike")
                    .help("Strike price")
                    .value_name("PRICE"),
            )
            .arg(
                Arg::new("range")
                    .long("range")
                    .help("Range: ITM, NTM, OTM, etc.")
                    .value_name("RANGE"),
            )
            .arg(
                Arg::new("from-date")
                    .long("from-date")
                    .help("From date (yyyy-MM-dd)")
                    .value_name("DATE"),
            )
            .arg(
                Arg::new("to-date")
                    .long("to-date")
                    .help("To date (yyyy-MM-dd)")
                    .value_name("DATE"),
            )
            .arg(
                Arg::new("volatility")
                    .long("volatility")
                    .help("Volatility for ANALYTICAL strategy")
                    .value_name("VOL"),
            )
            .arg(
                Arg::new("underlying-price")
                    .long("underlying-price")
                    .help("Underlying price for ANALYTICAL strategy")
                    .value_name("PRICE"),
            )
            .arg(
                Arg::new("interest-rate")
                    .long("interest-rate")
                    .help("Interest rate for ANALYTICAL strategy")
                    .value_name("RATE"),
            )
            .arg(
                Arg::new("days-to-expiration")
                    .long("days-to-expiration")
                    .help("Days to expiration for ANALYTICAL strategy")
                    .value_name("DAYS"),
            )
            .arg(
                Arg::new("exp-month")
                    .long("exp-month")
                    .help("Expiration month")
                    .value_name("MONTH"),
            )
            .arg(
                Arg::new("option-type")
                    .long("option-type")
                    .help("Option type")
                    .value_name("TYPE"),
            ),
    ]
}
