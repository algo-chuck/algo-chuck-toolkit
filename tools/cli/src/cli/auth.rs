use clap::Command;

/// Build authentication-related commands (login, refresh, status)
pub fn auth_commands() -> Vec<Command> {
    vec![
        Command::new("login").about("Perform initial OAuth2 authentication"),
        Command::new("refresh").about("Refresh access token using refresh token"),
        Command::new("status").about("Display current token status and expiry times"),
    ]
}
