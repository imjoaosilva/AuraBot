use serenity::all::{Context, Interaction};

use super::{commands, components};

pub async fn run(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        match command.data.name.as_str() {
            "meta" => commands::meta::run(ctx, command).await,
            "canal" => commands::canal::run(ctx, command).await,
            "anonimo" => commands::anonimo::run(ctx, command).await,
            "definirmeta" => commands::definirmeta::run(ctx, command).await,
            "definircanais" => commands::definircanais::run(ctx, command).await,
            "definircanal" => commands::definircanal::run(ctx, command).await,
            "info" => commands::info::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        }
    } else if let Interaction::Modal(interaction) = interaction {
        match interaction.data.custom_id.as_str() {
            "goal" => components::modal_goal::run(ctx, interaction).await,
            _ => println!("❌ - Modal not found!"),
        }
    } else if let Interaction::Component(interaction) = interaction {
        match interaction.data.custom_id.as_str() {
            "aprove" => components::buttons_meta::run(ctx, interaction, "Approved").await,
            "deny" => components::buttons_meta::run(ctx, interaction, "Rejected").await,
            _ => println!("❌ - Button not found!"),
        }
    }
}
