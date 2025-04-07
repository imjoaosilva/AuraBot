use serenity::all::{
    ChannelId, Colour, CommandInteraction, Context, CreateChannel, CreateCommand, CreateEmbed,
    CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
    InteractionResponseFlags, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId,
    Timestamp,
};
use std::env;

use super::utils;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let category_env = env::var("INDIVIDUAL_CATEGORY_ID")
        .expect("❌ - Category ID not found!")
        .parse()
        .expect("❌ - Category ID must be an integer");

    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId::new(command.guild_id.unwrap().into())),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(command.user.id),
        },
    ];

    let channel = CreateChannel::new(format!("🙋┇{}", command.user.name))
        .permissions(permissions)
        .category(ChannelId::new(category_env));

    let created_channel = command
        .guild_id
        .unwrap()
        .create_channel(&ctx.http, channel)
        .await
        .unwrap();

    let embed_description = format!(
        "- Olá <@{}>, o seu novo canal individual foi aberto.\n> Você pode encontrar ele aqui <#{}>.",
        command.user.id, created_channel.id
    );

    let embed_footer = CreateEmbedFooter::new("Aura - Canal Individual");

    let embed = CreateEmbed::default()
        .description(embed_description)
        .footer(embed_footer)
        .timestamp(Timestamp::now())
        .colour(Colour::LIGHT_GREY);

    let reply_data = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    command
        .create_response(&ctx.http, reply_builder)
        .await
        .unwrap();

    utils::logs::send_log(
        &ctx,
        format!(
            "Canal individual criado com sucesso!\n> Canal: <#{}>\n> Criador: <@{}>",
            created_channel.id, command.user.id
        ),
    )
    .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("canal").description("Crie o seu canal individual")
}
