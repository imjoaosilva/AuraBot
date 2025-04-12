use super::{models::client::ClientData, utils};
use serenity::all::{
    ChannelId, Colour, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, InteractionResponseFlags, Timestamp,
};

pub async fn run(ctx: Context, interaction: ComponentInteraction, status: &str) {
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

    repo.update_meta_status(interaction.message.id.get(), status.to_string())
        .await
        .unwrap_or_else(|_| {
            eprintln!("❌ - Failed to update meta status.");
        });

    let description = if status == "Approved" {
        "✅ **Meta Aprovada com Sucesso!**"
    } else {
        "❌ **Meta Rejeitada com Sucesso!**"
    };

    let reply = CreateInteractionResponseMessage::new()
        .add_embed(
            CreateEmbed::default()
                .description(description)
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

    let Ok(metas) = repo.get_user_approved_weekly(meta.user_id).await else {
        eprintln!("❌ - Failed to fetch approved metas.");
        return;
    };

    let total: i32 = metas.iter().map(|meta| meta.amount as i32).sum();

    let user_embed_title = if status == "Approved" {
        "Meta Aprovada"
    } else {
        "Meta Reprovada"
    };

    let user_embed_description = if status == "Approved" {
        format!(
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
        )
    } else {
        format!(
            "A sua meta foi reprovada com sucesso!\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━\n\
            💰 **Valor Reprovado:** `{}`\n\
            ❌ **Status:** `Reprovada`\n\
            🛡️ **Reprovado por:** <@{}>\n\
            📊 **Valor Restante:** `{}`\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━",
            utils::format_amount(meta.amount as u64),
            interaction.user.id.get(),
            if total as i64 > current_meta {
                String::from("0")
            } else {
                utils::format_amount((current_meta - total as i64) as u64)
            }
        )
    };

    let user_embed = CreateEmbed::default()
        .title(user_embed_title)
        .description(user_embed_description)
        .timestamp(Timestamp::now())
        .colour(Colour::LIGHT_GREY);

    let user_builder = CreateMessage::default().add_embed(user_embed);

    if let Err(err) = ChannelId::new(user_channel_id as u64)
        .send_message(&ctx.http, user_builder)
        .await
    {
        eprintln!("❌ - Failed to send anonymous message: {}", err);
        return;
    }

    let logs_message = if status == "Approved" {
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
        )
    } else {
        format!(
            "❌ **Meta Reprovada**\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━\n\
            👤 **Entregue por:** <@{}>\n\
            💰 **Valor da Meta:** `{}`\n\
            ❌ **Status:** `Reprovada`\n\
            🛡️ **Reprovado por:** <@{}>\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━",
            meta.user_id,
            utils::format_amount(meta.amount as u64),
            interaction.user.id.get()
        )
    };

    utils::logs::send_log(
        &ctx,
        logs_message,
        channels.logs_channel_id,
    )
    .await;
}
