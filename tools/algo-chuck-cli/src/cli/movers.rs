use clap::{Arg, Command};

pub fn movers_commands() -> Vec<Command> {
    vec![
        Command::new("movers")
            .about("Get movers for a specific index")
            .arg(
                Arg::new("symbol")
                    .help("Index symbol ($DJI, $COMPX, $SPX, etc.)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("sort")
                    .long("sort")
                    .help("Sort by: VOLUME, TRADES, PERCENT_CHANGE_UP, PERCENT_CHANGE_DOWN")
                    .value_name("SORT"),
            )
            .arg(
                Arg::new("frequency")
                    .long("frequency")
                    .help("Frequency: 0, 1, 5, 10, 30, 60")
                    .value_name("FREQ"),
            ),
    ]
}
