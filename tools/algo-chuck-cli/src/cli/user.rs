use clap::Command;

/// Build user-related commands
pub fn user_commands() -> Vec<Command> {
    vec![
        Command::new("user-preference")
            .about("Get user preference information for the logged in user."),
    ]
}
