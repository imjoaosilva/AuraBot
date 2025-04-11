use serenity::all::{Context, Interaction};

use super::{commands, components};

pub async fn run(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        match command.data.name.as_str() {
            "meta" => commands::meta::run(ctx, command).await,
            "canal" => commands::canal::run(ctx, command).await,
            "anonimo" => commands::anonimo::run(ctx, command).await,
            "setmeta" => commands::setmeta::run(ctx, command).await,
            "setcanais" => commands::setcanais::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        }
    }
    else if let Interaction::Modal(interaction) = interaction {
        match interaction.data.custom_id.as_str() {
            "goal" => components::modal_goal::run(ctx, interaction).await,
            _ => println!("❌ - Modal not found!"),
        }
    }
}
