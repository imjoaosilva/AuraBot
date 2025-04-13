use super::models::client::ClientData;
use super::utils;
use serenity::all::{
    ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateAllowedMentions,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, InteractionResponseFlags, Permissions,
    Timestamp,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Error getting channels.");
        return;
    };

    let Some(amount) = command
        .data
        .options
        .first()
        .and_then(|val| val.value.as_i64())
    else {
        eprintln!("❌ - Opção de quantidade inválida ou ausente.");
        return;
    };

    if amount < 0 {
        let reply = CreateInteractionResponseMessage::new()
            .content("🚫 O valor da meta não pode ser negativo!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        let _ = command
            .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
            .await;
        return;
    }

    if let Err(err) = repo.set_meta(amount).await {
        eprintln!("❌ - Falha ao definir meta: {}", err);
        return;
    }

    let short_amount = utils::format_amount(amount as u64);
    let footer = CreateEmbedFooter::new(format!("Meta definida por {}", command.user.name));
    let timestamp = utils::get_next_monday_at_18().timestamp();

    let public_embed = CreateEmbed::default()
        .title("📢 Nova Meta Semanal Ativada!")
        .description(format!(
            "**💰 Valor da meta:** `{} sujo`\n\
             **📅 Data Limite:** <t:{}:R>\n\n\
             Quem será o destaque da semana? 👀",
            short_amount, timestamp
        ))
        .footer(footer)
        .timestamp(Timestamp::now())
        .colour(Colour::from_rgb(241, 196, 15));

    let channel = ChannelId::new(channels.meta_channel_id);

    let _ = channel
        .send_message(&ctx.http, CreateMessage::default().add_embed(public_embed))
        .await
        .map_err(|err| eprintln!("❌ - Falha ao enviar embed público: {}", err));

    let _ = channel
        .send_message(
            &ctx.http,
            CreateMessage::default()
                .content("@everyone")
                .allowed_mentions(CreateAllowedMentions::new().everyone(true)),
        )
        .await
        .map_err(|err| eprintln!("❌ - Falha ao mencionar everyone: {}", err));

    let embed = CreateEmbed::default()
        .title("✅ Meta Atualizada")
        .description(format!(
            "📌 **Meta semanal definida com sucesso!**\n\n💰 Valor definido: **${}**\n🗓️ Vigência: *Esta semana*\n\nVamos com tudo! 🚀",
            short_amount
        ))
        .timestamp(Timestamp::now())
        .colour(Colour::from_rgb(46, 204, 113));

    let reply = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let _ = command
        .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
        .await
        .map_err(|err| eprintln!("❌ - Falha ao enviar resposta: {}", err));

    let log = format!(
        "📢 **Meta semanal atualizada!**\n> 💸 Valor: **${}**\n> 👤 Responsável: <@{}>",
        short_amount, command.user.id
    );

    utils::logs::send_log(&ctx, log, channels.logs_channel_id).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("definirmeta")
        .description("Defina a meta semanal de dinheiro sujo")
        .set_options(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "quantidade",
            "A quantidade de dinheiro sujo (ex: 1000000).",
        )
        .required(true)])
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
