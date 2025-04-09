use serenity::all::{
    ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, InteractionResponseFlags, Timestamp,
};
use std::env;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let channel_id = match env::var("ANONYMOUS_CHANNEL_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok())
    {
        Some(id) => id,
        None => {
            eprintln!("❌ - ANONYMOUS_CHANNEL_ID not found or invalid.");
            return;
        }
    };

    let channel = ChannelId::new(channel_id);

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

    if let Err(err) = channel.send_message(&ctx.http, builder).await {
        eprintln!("❌ - Failed to send anonymous message: {}", err);
        return;
    }

    let confirmation = CreateEmbed::default()
        .description("A sua mensagem foi enviada com sucesso!")
        .colour(Colour::LIGHT_GREY);

    let reply_data = CreateInteractionResponseMessage::new()
        .add_embed(confirmation)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    let _ = command.create_response(&ctx.http, reply_builder).await;
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
