use clap::{Arg, Command};

/// Build CA certificate management command
pub fn ca_command() -> Command {
    Command::new("ca")
        .about("Manage Certificate Authority for seamless HTTPS")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("status").about("Show CA certificate status"))
        .subcommand(Command::new("install").about("Install CA certificate in system trust store"))
        .subcommand(
            Command::new("uninstall").about("Remove CA certificate from system trust store"),
        )
        .subcommand(Command::new("regenerate").about("Regenerate CA certificate"))
        .subcommand(Command::new("show").about("Display CA certificate for manual installation"))
        .subcommand(
            Command::new("clean")
                .about("Remove all CA certificates and optionally uninstall from system")
                .arg(
                    Arg::new("uninstall")
                        .long("uninstall")
                        .help("Also remove CA from system trust store")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("test")
                .about("Test certificate by opening browser to HTTPS server")
                .arg(
                    Arg::new("port")
                        .long("port")
                        .help("Port to run the test server on")
                        .default_value("8443")
                        .value_parser(clap::value_parser!(u16)),
                ),
        )
}
