use clap::{Arg, Command};

pub fn options_expiration_chain_commands() -> Vec<Command> {
    vec![
        Command::new("expiration-chain")
            .about("Get option expiration chain for an optionable symbol")
            .arg(
                Arg::new("symbol")
                    .help("Symbol (e.g., AAPL)")
                    .required(true)
                    .index(1),
            ),
    ]
}
