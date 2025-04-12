use super::models::client::ClientData;
use super::utils;
use chrono_tz::America::Sao_Paulo;
use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, InteractionResponseFlags, Timestamp,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    let Ok(current_meta) = repo.get_meta().await else {
        eprintln!("❌ - Failed to fetch current meta.");
        return;
    };

    let Ok(metas) = repo
        .get_user_approved_weekly(command.user.id.get() as i64)
        .await
    else {
        eprintln!("❌ - Failed to fetch approved metas.");
        return;
    };

    let total: i32 = metas.iter().map(|meta| meta.amount as i32).sum();

    let embed = CreateEmbed::default()
        .colour(Colour::from_rgb(144, 238, 144))
        .title("📊 Informações da Meta Semanal")
        .description(format!(
            "💰 **Meta Atual:** `{}`\n\n\
            📈 **Total Entregue:** `{}`\n\n\
            📅 **Data de entrega:** <t:{}:R>",
            utils::format_amount(current_meta as u64),
            utils::format_amount(total as u64),
            utils::get_next_monday_at_18().with_timezone(&Sao_Paulo).timestamp()
        ))
        .footer(CreateEmbedFooter::new("Meta Semanal"))
        .timestamp(Timestamp::now());

    let builder = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let _ = command
        .create_response(&ctx.http, CreateInteractionResponse::Message(builder))
        .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("Ver informações sobre a sua meta semanal")
}
