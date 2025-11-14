use clap::{Arg, Command};

pub fn instruments_commands() -> Vec<Command> {
    vec![
        Command::new("instruments")
            .about("Get instruments by symbol and projection")
            .arg(
                Arg::new("symbol")
                    .help("Symbol to search for")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("projection")
                    .help("Projection: symbol-search, symbol-regex, desc-search, desc-regex, search, fundamental")
                    .required(true)
                    .index(2),
            ),
        Command::new("instrument")
            .about("Get instrument by CUSIP")
            .arg(
                Arg::new("cusip")
                    .help("CUSIP identifier")
                    .required(true)
                    .index(1),
            ),
    ]
}
