use serenity::all::CreateCommand;
use super::commands;

pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        commands::meta::register(),
        commands::canal::register(),
        // commands::topstaffs::register(),
    ]
}