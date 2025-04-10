use super::commands;
use serenity::all::CreateCommand;

pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        commands::meta::register(),
        commands::canal::register(),
        commands::anonimo::register(),
        commands::setmeta::register(),
    ]
}
