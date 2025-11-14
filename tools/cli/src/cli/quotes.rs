use clap::{Arg, ArgAction, Command};

pub fn quote_commands() -> Vec<Command> {
    vec![
        Command::new("quotes")
            .about("Get quotes for multiple symbols")
            .arg(
                Arg::new("symbols")
                    .help("Comma-separated list of symbols (e.g., AAPL,MSFT,GOOGL)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("fields")
                    .long("fields")
                    .help("Fields to request (quote, fundamental, extended, reference, regular)")
                    .value_name("FIELDS"),
            )
            .arg(
                Arg::new("indicative")
                    .long("indicative")
                    .help("Include indicative symbol quotes for all ETF symbols")
                    .action(ArgAction::SetTrue),
            ),
        Command::new("quote")
            .about("Get quote for a single symbol")
            .arg(
                Arg::new("symbol")
                    .help("Symbol to get quote for (e.g., AAPL)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("fields")
                    .long("fields")
                    .help("Fields to request (quote, fundamental, extended, reference, regular)")
                    .value_name("FIELDS"),
            ),
    ]
}
