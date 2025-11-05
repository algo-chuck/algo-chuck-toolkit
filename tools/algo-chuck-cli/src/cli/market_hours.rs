use clap::{Arg, Command};

pub fn market_hours_commands() -> Vec<Command> {
    vec![
        Command::new("markets")
            .about("Get market hours for multiple markets")
            .arg(
                Arg::new("markets")
                    .help("Comma-separated markets (equity,option,bond,future,forex)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("date")
                    .long("date")
                    .help("Date (yyyy-MM-dd)")
                    .value_name("DATE"),
            ),
        Command::new("market")
            .about("Get market hours for a single market")
            .arg(
                Arg::new("market")
                    .help("Market: equity, option, bond, future, forex")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("date")
                    .long("date")
                    .help("Date (yyyy-MM-dd)")
                    .value_name("DATE"),
            ),
    ]
}
