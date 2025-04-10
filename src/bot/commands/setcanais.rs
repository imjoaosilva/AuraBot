use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    InteractionResponseFlags, Timestamp,
};

use super::models::client::ClientData;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let options = command.data.options.clone();
    let individual_channel_id = options[0].value.as_channel_id().unwrap();
    let anonimo_channel_id = options[1].value.as_channel_id().unwrap();
    let meta_channel_id = options[2].value.as_channel_id().unwrap();
    let logs_channel_id = options[3].value.as_channel_id().unwrap();

    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    if let Err(err) = repo
        .set_channels(
            individual_channel_id.get(),
            anonimo_channel_id.get(),
            meta_channel_id.get(),
            logs_channel_id.get(),
        )
        .await
    {
        eprintln!("❌ - Failed to set channels: {}", err);
        return;
    }

    let embed = CreateEmbed::default()
        .description("Canais definidos com sucesso!")
        .colour(Colour::LIGHT_GREY)
        .timestamp(Timestamp::now());

    let reply_data = CreateInteractionResponseMessage::new()
        .add_embed(embed)
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    if let Err(err) = command.create_response(&ctx.http, reply_builder).await {
        eprintln!("❌ - Failed to send response: {}", err);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setcanais")
        .description("Defina os canais do bot")
        .set_options(vec![
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "individual",
                "Categoria para canais individuais",
            )
            .required(true),
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "anonimo",
                "Canal para mensagens anónimas",
            )
            .required(true),
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "meta",
                "Canal para mensagens de meta",
            )
            .required(true),
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "logs",
                "Canal para receber logs",
            )
            .required(true),
        ])
}
