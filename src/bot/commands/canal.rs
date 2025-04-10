use serenity::all::{
    ChannelId, Colour, CommandInteraction, Context, CreateChannel, CreateCommand, CreateEmbed,
    CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
    InteractionResponseFlags, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId,
    Timestamp,
};

use super::models::client::ClientData;
use super::utils;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    if let Ok(Some(existing_channel_id)) = repo.get_user_channel(command.user.id.get()).await {
        let reply_data = CreateInteractionResponseMessage::new()
            .content(format!(
                "❌ - Você já possui um canal individual aberto <#{}>.",
                existing_channel_id
            ))
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply_builder = CreateInteractionResponse::Message(reply_data);

        if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
            eprintln!("❌ - Failed to send response: {}", err);
        }

        return;
    }

    let Some(guild_id) = command.guild_id else {
        eprintln!("❌ - Guild ID not found.");
        return;
    };

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Falha ao obter os canais.");
        return;
    };

    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId::new(guild_id.get())),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(command.user.id),
        },
    ];

    let channel = CreateChannel::new(format!("🙋┇{}", command.user.name))
        .permissions(permissions)
        .category(ChannelId::new(channels.individuals_category_id));

    let Ok(created_channel) = guild_id.create_channel(&ctx.http, channel).await else {
        eprintln!("❌ - Failed to create channel.");
        return;
    };

    if let Err(err) = repo
        .create_user_channel(command.user.id.get(), created_channel.id.get())
        .await
    {
        eprintln!("❌ - Failed to create user channel in database: {}", err);
        return;
    }

    let embed = CreateEmbed::default()
        .description(format!(
            "- Olá <@{}>, o seu novo canal individual foi aberto.\n> Você pode encontrar ele aqui <#{}>.",
            command.user.id, created_channel.id
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
            "Canal individual criado com sucesso!\n> Canal: <#{}>\n> Criador: <@{}>",
            created_channel.id, command.user.id
        ),
        channels.logs_channel_id
    )
    .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("canal").description("Crie o seu canal individual")
}
