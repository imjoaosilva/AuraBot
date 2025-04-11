use super::{models::client::ClientData, utils};
use serenity::all::{
    ChannelId, Colour, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, InteractionResponseFlags,
};

pub async fn run(ctx: Context, interaction: ComponentInteraction) {
    let mut data = ctx.data.write().await;
    let repo = match data.get_mut::<ClientData>() {
        Some(repo) => repo,
        None => {
            eprintln!("❌ - Error accessing client data.");
            return;
        }
    };

    let Ok(channels) = repo.get_channels().await else {
        eprintln!("❌ - Failed to fetch channels.");
        return;
    };

    let Ok(meta) = repo
        .get_user_meta_by_message_id(interaction.message.id.get())
        .await
    else {
        eprintln!("❌ - Failed to fetch meta.");
        return;
    };

    repo.update_meta_status(interaction.message.id.get(), "Approved".to_string())
        .await
        .unwrap_or_else(|_| {
            eprintln!("❌ - Failed to update meta status.");
        });

    let reply = CreateInteractionResponseMessage::new()
        .add_embed(
            CreateEmbed::default()
                .description("✅ **Meta Aprovada com Sucesso!**")
                .colour(Colour::LIGHT_GREY),
        )
        .flags(InteractionResponseFlags::EPHEMERAL);

    let _ = interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(reply))
        .await;

    let Ok(user_channel_id) = repo.get_user_channel(meta.user_id as u64).await else {
        eprintln!("❌ - Failed to fetch user channel.");
        return;
    };

    let Some(user_channel_id) = user_channel_id else {
        eprintln!("❌ - User's channel not found.");
        return;
    };

    let Ok(current_meta) = repo.get_meta().await else {
        eprintln!("❌ - Failed to fetch current meta.");
        return;
    };

    let Ok(metas) = repo.get_approved_metas_from_current_week().await else {
        eprintln!("❌ - Failed to fetch approved metas.");
        return;
    };

    let total: i32 = metas.iter().map(|meta| meta.amount as i32).sum();

    let user_embed = CreateEmbed::default()
        .title("Meta Aprovada")
        .description(format!(
            "A sua meta foi aprovada com sucesso!\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━\n\
            💰 **Valor Aprovado:** `{}`\n\
            ✅ **Status:** `Aprovada`\n\
            🛡️ **Aprovado por:** <@{}>\n\
            📊 **Valor Restante:** `{}`\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━",
            utils::format_amount(meta.amount as u64),
            interaction.user.id.get(),
            if total as i64 > current_meta {
                String::from("0")
            } else {
                utils::format_amount((current_meta - total as i64) as u64)
            }
        ))
        .colour(Colour::LIGHT_GREY);

    let user_builder = CreateMessage::default().add_embed(user_embed);

    if let Err(err) = ChannelId::new(user_channel_id as u64)
        .send_message(&ctx.http, user_builder)
        .await
    {
        eprintln!("❌ - Failed to send anonymous message: {}", err);
        return;
    }

    utils::logs::send_log(
        &ctx,
        format!(
            "🎯 **Meta Entregue com Sucesso!**\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━\n\
            👤 **Entregue por:** <@{}>\n\
            💰 **Valor Entregue:** `{}`\n\
            ✅ **Status:** `Aprovada`\n\
            🛡️ **Aprovado por:** <@{}>\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━",
            meta.user_id,
            utils::format_amount(meta.amount as u64),
            interaction.user.id.get()
        ),
        channels.logs_channel_id,
    )
    .await;
}
