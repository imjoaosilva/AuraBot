use chrono_tz::America::Sao_Paulo;
use serenity::all::{ChannelId, Context, CreateMessage};
use std::sync::Arc;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

use super::models::client::ClientData;
use crate::bot::utils;

pub async fn setup_cron_jobs(ctx: Arc<Context>) {
    let scheduler = JobScheduler::new()
        .await
        .expect("❌ Failed to create scheduler");

    let job = JobBuilder::new()
        .with_timezone(Sao_Paulo)
        .with_cron_job_type()
        .with_schedule("0 45 15 * * 6")
        .unwrap()
        .with_run_async(Box::new(move |_uuid, _l| {
            let ctx = ctx.clone();
            println!("🕐 - Running job to send reminder messages");

            Box::pin(async move {
                let data = ctx.data.read().await;
                let repo = data.get::<ClientData>().unwrap();
                let users_results = repo.get_all_user_channels().await;
                let Ok(user_channels) = users_results else {
                    println!("❌ Failed to fetch approved metas");
                    return;
                };

                if user_channels.is_empty() {
                    println!("ℹ️ No metas to send");
                    return;
                }

                let Ok(current_meta) = repo.get_meta().await else {
                    eprintln!("❌ - Failed to fetch current meta.");
                    return;
                };

                for (user_id, channel_id) in user_channels {
                    println!("🕐 - Sending reminder to user: {}", user_id);
                    println!("🕐 - Sending reminder to channel: {}", channel_id);

                    let Ok(metas_for_user) = repo.get_user_approved_weekly(user_id as i64).await
                    else {
                        eprintln!("❌ - Failed to fetch approved metas.");
                        return;
                    };

                    let total: i32 = metas_for_user.iter().map(|meta| meta.amount as i32).sum();

                    if let Err(e) =
                        send_message(&ctx, channel_id, user_id, total as i64, current_meta as i64)
                            .await
                    {
                        eprintln!("❌ - Failed to send message: {:?}", e);
                    }
                }
            })
        }))
        .build()
        .unwrap();

    scheduler
        .add(job)
        .await
        .expect("❌ Failed to add scheduled job");

    scheduler
        .start()
        .await
        .expect("❌ Failed to start scheduler");
}

async fn send_message(
    ctx: &Context,
    channel_id: u64,
    user_id: u64,
    total: i64,
    current_meta: i64,
) -> Result<(), serenity::Error> {
    let content = format!(
        "<@{}> Você foi **AVISADO**. Faltam **3 dias** para o fim do prazo de entrega e falta `{}` a ser entregue.",
        user_id,
        if total as i64 > current_meta {
            String::from("0")
        } else {
            utils::format_amount((current_meta - total as i64) as u64)
        }
    );

    let channel = ChannelId::new(channel_id);
    let builder = CreateMessage::default().content(content);

    if let Err(e) = channel.send_message(&ctx.http, builder).await {
        println!("❌ Failed to send message to user channel: {:?}", e);
    }
    Ok(())
}
