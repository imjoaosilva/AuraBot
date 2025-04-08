use serenity::all::{Context, Interaction};
// use crate::buttons;

use super::commands;

pub async fn run(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        match command.data.name.as_str() {
            "meta" => commands::meta::run(ctx, command).await,
            "canal" => commands::canal::run(ctx, command).await,
            "anonimo" => commands::anonimo::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        }
    }
}