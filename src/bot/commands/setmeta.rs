use std::env;

use super::models::client::ClientData;
use super::utils;
use serenity::all::{
    ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateAllowedMentions,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, InteractionResponseFlags, Timestamp,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let channel_id = match env::var("META_CHANNEL_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok())
    {
        Some(id) => id,
        None => {
            eprintln!("❌ - META_CHANNEL_ID not found or invalid.");
            return;
        }
    };

    let channel = ChannelId::new(channel_id);

    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();
    let Some(amount) = command
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_i64())
    else {
        eprintln!("❌ - Missing or invalid message option.");
        return;
    };

    if amount < 0 {
        let reply_data = CreateInteractionResponseMessage::new()
            .content("🚫 O valor da meta não pode ser negativo!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply_builder = CreateInteractionResponse::Message(reply_data);

        if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
            eprintln!("❌ - Failed to send response: {}", err);
        }
        return;
    }

    repo.set_meta(amount)
        .await
        .expect("❌ - Failed to set meta");

    let short_amount = utils::format_amount(amount as u64);

    let footer = CreateEmbedFooter::new(format!("Meta definida por {}", command.user.name));

    let next_monday_at_18 = utils::get_next_monday_at_18();
    let timestamp_next_monday = next_monday_at_18.timestamp();

    let public_embed = CreateEmbed::default()
        .title("📢 Nova Meta Semanal Ativada!")
        .description(format!(
            "**💰 Valor da meta:** `{} sujo`\n\
        **📅 Data Limite:** <t:{}:R>\n\n\
        Quem será o destaque da semana? 👀",
            short_amount, timestamp_next_monday
        ))
        .footer(footer)
        .timestamp(Timestamp::now())
        .colour(Colour::from_rgb(241, 196, 15));

    let public_builder = CreateMessage::default().add_embed(public_embed);

    if let Err(err) = channel.send_message(&ctx.http, public_builder).await {
        eprintln!("❌ - Failed to send public embed to meta channel: {}", err);
    }

    let everyone = CreateMessage::default()
        .content("@everyone")
        .allowed_mentions(CreateAllowedMentions::new().everyone(true));

    if let Err(err) = channel.send_message(&ctx.http, everyone).await {
        eprintln!("❌ - Failed to send public embed to meta channel: {}", err);
    }

    let embed_description = format!(
        "📌 **Meta semanal definida com sucesso!**\n\n💰 Valor definido: **${}**\n🗓️ Vigência: *Esta semana*\n\nVamos com tudo! 🚀",
        short_amount
    );

    let embed = CreateEmbed::default()
        .title("✅ Meta Atualizada")
        .description(embed_description)
        .timestamp(Timestamp::now())
        .colour(Colour::from_rgb(46, 204, 113));

    let reply_data = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
        eprintln!("❌ - Failed to send response: {}", err);
    }

    let log_message = format!(
        "📢 **Meta semanal atualizada!**\n> 💸 Valor: **${}**\n> 👤 Responsável: <@{}>",
        short_amount, command.user.id
    );

    utils::logs::send_log(&ctx, log_message).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setmeta")
        .description("Defina a meta semanal de dinheiro sujo")
        .set_options(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "quantidade",
            "A quantidade de dinheiro sujo (ex: 1000000).",
        )
        .required(true)])
}
