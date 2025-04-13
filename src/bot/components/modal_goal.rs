use super::{models::client::ClientData, utils};
use serenity::all::{
    ActionRow, ActionRowComponent, ButtonStyle, ChannelId, Colour, Context, CreateActionRow,
    CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, InteractionResponseFlags, ModalInteraction, Timestamp,
};

pub async fn run(ctx: Context, interaction: ModalInteraction) {
    let mut data = ctx.data.write().await;
    let repo = match data.get_mut::<ClientData>() {
        Some(repo) => repo,
        None => {
            eprintln!("Error: ClientData not found.");
            return;
        }
    };

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Error getting channels.");
        return;
    };

    let components = &interaction.data.components;
    let inputs: Vec<String> = components.iter().flat_map(|row| get_texts(row)).collect();

    if inputs.len() < 2 {
        println!("Error: Not all fields were filled.");
        return;
    }

    let value = &inputs[0];
    let responsible = &inputs[1];

    if let Ok(value) = value.parse::<f64>() {
        match repo.get_user_last_goal(interaction.user.id.get()).await {
            Ok(Some(last_goal)) => {
                if last_goal.status == Some("Pending".to_string()) {
                    let reply = CreateInteractionResponseMessage::new()
                        .content("🚫 A sua ultima meta enviada ainda não foi analisada.")
                        .flags(InteractionResponseFlags::EPHEMERAL);

                    if let Err(err) = interaction
                        .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
                        .await
                    {
                        eprintln!("Error sending response: {}", err);
                    }
                    return;
                }
            }
            Ok(None) | Err(_) => {}
        }

        let embed = CreateEmbed::default()
            .colour(Colour::from_rgb(144, 238, 144))
            .title("📊 Pedido de Entrega de Meta")
            .description(format!(
                "🚀 **Novo Pedido de Meta Recebido!**\n\n\
            👤 **Usuário:** <@{}>\n\
            👤 **Responsável:** `{}`\n\
            💰 **Valor da Meta:** `{}`\n\n\
            Por favor, avalie e processe este pedido com atenção. ✅",
                interaction.user.id.get(),
                responsible,
                utils::format_amount(value as u64)
            ))
            .timestamp(Timestamp::now());

        let aprove = CreateButton::new("aprove")
            .label("Aprovar")
            .style(ButtonStyle::Success);

        let deny = CreateButton::new("deny")
            .label("Recusar")
            .style(ButtonStyle::Danger);

        let action_row = Vec::from([CreateActionRow::Buttons(vec![aprove, deny])]);

        let message_builder = CreateMessage::new().embed(embed).components(action_row);

        let channel = ChannelId::new(channels.approval_channel_id);

        let Ok(created_message) = channel.send_message(&ctx.http, message_builder).await else {
            eprintln!("❌ - Failed to send message.");
            return;
        };

        if let Err(err) = repo
            .create_goal(
                interaction.user.id.get(),
                value as i64,
                created_message.id.get().to_string(),
            )
            .await
        {
            eprintln!("❌ - Failed to create goal in database: {}", err);
            return;
        }

        let reply = CreateInteractionResponseMessage::new()
            .content("✅ A tua meta foi enviada com sucesso! Em breve será analisada.")
            .flags(InteractionResponseFlags::EPHEMERAL);

        if let Err(err) = interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
            .await
        {
            eprintln!("Error sending response: {}", err);
        }
    } else {
        let reply = CreateInteractionResponseMessage::new()
            .content("🚫 The goal value must be a valid number!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        if let Err(err) = interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
            .await
        {
            eprintln!("Error sending response: {}", err);
        }
    }
}

fn get_texts(action_row: &ActionRow) -> Vec<String> {
    action_row
        .components
        .iter()
        .filter_map(|component| {
            if let ActionRowComponent::InputText(input_text) = component {
                input_text.value.clone()
            } else {
                None
            }
        })
        .collect()
}
