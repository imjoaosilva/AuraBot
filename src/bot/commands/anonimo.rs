use serenity::all::{
    ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, InteractionResponseFlags, Timestamp,
};

use super::models::client::ClientData;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Error getting channels.");
        return;
    };

    let Some(user_message) = command
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_str())
    else {
        eprintln!("❌ - Missing or invalid message option.");
        return;
    };

    let embed = CreateEmbed::default()
        .description(user_message)
        .timestamp(Timestamp::now())
        .colour(Colour::LIGHT_GREY);

    let builder = CreateMessage::default().add_embed(embed);

    if let Err(err) = ChannelId::new(channels.anonymous_channel_id)
        .send_message(&ctx.http, builder)
        .await
    {
        eprintln!("❌ - Falha ao enviar mensagem anónima: {}", err);
        return;
    }

    let reply = CreateInteractionResponseMessage::new()
        .add_embed(
            CreateEmbed::default()
                .description("A sua mensagem foi enviada com sucesso!")
                .colour(Colour::LIGHT_GREY),
        )
        .flags(InteractionResponseFlags::EPHEMERAL);

    let _ = command
        .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
        .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("anonimo")
        .description("Envie uma mensagem anónima")
        .set_options(vec![CreateCommandOption::new(
            CommandOptionType::String,
            "mensagem",
            "Mande uma mensagem anónima.",
        )
        .required(true)])
}
