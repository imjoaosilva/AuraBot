use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    InteractionResponseFlags, Permissions, Timestamp,
};

use super::models::client::ClientData;

pub async fn run(ctx: Context, command: CommandInteraction) {
    let options = command.data.options.clone();
    let individual_channel_id = options[0].value.as_channel_id().unwrap();
    let anonimo_channel_id = options[1].value.as_channel_id().unwrap();
    let meta_channel_id = options[2].value.as_channel_id().unwrap();
    let logs_channel_id = options[3].value.as_channel_id().unwrap();
    let approval_channel_id = options[4].value.as_channel_id().unwrap();
    let resultadometa_channel_id = options[5].value.as_channel_id().unwrap();

    let data = ctx.data.read().await;
    let repo = data.get::<ClientData>().unwrap();

    if let Err(err) = repo
        .set_channels(
            individual_channel_id.get(),
            anonimo_channel_id.get(),
            meta_channel_id.get(),
            logs_channel_id.get(),
            approval_channel_id.get(),
            resultadometa_channel_id.get(),
        )
        .await
    {
        eprintln!("❌ - Failed to set channels: {}", err);
        return;
    }

    let embed = CreateEmbed::default()
        .title("✅ Configuração Concluída!")
        .description(format!(
            "Os canais foram configurados com sucesso!\n\n\
         **Canais Definidos:**\n\
         💬 **Individual:** <#{}>\n\
         🤫 **Anônimo:** <#{}>\n\
         📊 **Meta:** <#{}>\n\
         📑 **Logs:** <#{}>\n\
         ✅ **Aprovação:** <#{}>
         📈 **Resultado da Meta:** <#{}>\n",
            individual_channel_id.get(),
            anonimo_channel_id.get(),
            meta_channel_id.get(),
            logs_channel_id.get(),
            approval_channel_id.get(),
            resultadometa_channel_id.get()
        ))
        .colour(Colour::from_rgb(144, 238, 144))
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
    CreateCommand::new("definircanais")
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
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "approval",
                "Canal para aprovações",
            )
            .required(true),
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "resultadometa",
                "Canal para o resultado das meta",
            )
            .required(true),
        ])
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
