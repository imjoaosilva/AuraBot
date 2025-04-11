use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
    InteractionResponseFlags, Permissions, Timestamp,
};

use super::{models::client::ClientData, utils};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Falha ao obter os canais.");
        return;
    };

    let options = command.data.options.clone();
    let user_id = options[0].value.as_user_id().unwrap();
    let channel_id = options[1].value.as_channel_id().unwrap();

    if let Ok(Some(existing_channel_id)) = repo.get_user_channel(user_id.get()).await {
        let reply_data = CreateInteractionResponseMessage::new()
            .content(format!(
                "❌ - Esse usuário já possui um canal individual aberto <#{}>.",
                existing_channel_id
            ))
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply_builder = CreateInteractionResponse::Message(reply_data);

        if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
            eprintln!("❌ - Failed to send response: {}", err);
        }

        return;
    }

    if let Err(err) = repo
        .create_user_channel(user_id.get(), channel_id.get())
        .await
    {
        eprintln!("❌ - Failed to create user channel in database: {}", err);
        return;
    }

    let embed = CreateEmbed::default()
        .description(format!(
            "- Você definiu o usuário <@{}> para o canal <#{}>",
            user_id.get(),
            channel_id.get()
        ))
        .footer(CreateEmbedFooter::new("Aura - Canal Individual"))
        .timestamp(Timestamp::now())
        .colour(Colour::LIGHT_GREY);

    let reply_data = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
        eprintln!("❌ - Failed to send response: {}", err);
    }

    utils::logs::send_log(
        &ctx,
        format!(
            "Canal individual definido com sucesso!\n> Canal: <#{}>\n> User: <@{}> \n> Definido por: <@{}>",
            channel_id.get(),
            user_id.get(),
            command.user.id
        ),
        channels.logs_channel_id,
    )
    .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("definircanal")
        .description("Defina um canal individual")
        .set_options(vec![
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "O usuário para o qual você deseja definir o canal",
            )
            .required(true),
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "canal",
                "O canal que você deseja definir",
            )
            .required(true),
        ])
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
