use clap::Command;

/// Build user-preference-related commands
pub fn user_preference_commands() -> Vec<Command> {
    vec![
        Command::new("user-preference")
            .about("Get user preference information for the logged in user."),
    ]
}
