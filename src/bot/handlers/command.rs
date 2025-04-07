use serenity::all::CreateCommand;
use super::commands;

pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        commands::meta::register(),
        // commands::mywhitelists::register(),
        // commands::topstaffs::register(),
    ]
}