use chrono_tz::America::Sao_Paulo;
use serenity::all::{ChannelId, Context, CreateMessage, UserId};
use std::{env, sync::Arc, time::Duration};
use tokio::time::sleep;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

use super::models::client::ClientData;
use crate::bot::utils;

pub async fn setup_cron_jobs(ctx: Arc<Context>) {
    let scheduler = JobScheduler::new()
        .await
        .expect("❌ Failed to create scheduler");

    let ctx_clone_1 = ctx.clone();
    let job = JobBuilder::new()
        .with_timezone(Sao_Paulo)
        .with_cron_job_type()
        .with_schedule("0 0 18 * * Sun,Sat")
        .unwrap()
        .with_run_async(Box::new(move |_uuid, _l| {
            let ctx = ctx_clone_1.clone();
            println!("🕐 - Running job to send reminder messages");

            Box::pin(async move {
                notify(&ctx, 3).await;
            })
        }))
        .build()
        .unwrap();

    let ctx_clone_2 = ctx.clone();

    let job02 = JobBuilder::new()
        .with_timezone(Sao_Paulo)
        .with_cron_job_type()
        .with_schedule("0 47 19 * * Mon")
        .unwrap()
        .with_run_async(Box::new(move |_uuid, _l| {
            let ctx = ctx_clone_2.clone();
            println!("🕐 - Running leaderboard job");

            let guild_id = env::var("GUILD_ID")
                .expect("❌ - GUILD_ID not set in environment variables")
                .parse::<u64>()
                .expect("❌ - Failed to parse GUILD_ID");

            Box::pin(async move {
                let data = ctx.data.read().await;
                let repo = data.get::<ClientData>().unwrap();

                let Ok(channels) = repo.get_channels().await else {
                    eprintln!("❌ - Falha ao obter os canais.");
                    return;
                };

                let Ok(metas) = repo.get_approved_metas_from_current_week().await else {
                    println!("❌ Failed to fetch approved metas");
                    return;
                };

                let Ok(current_meta) = repo.get_meta().await else {
                    println!("❌ Failed to fetch current meta");
                    return;
                };

                let Ok(all_users) = repo.get_all_user_channels().await else {
                    println!("❌ Failed to fetch all users");
                    return;
                };

                let mut leaderboard_data: Vec<(String, i64)> = Vec::new();
                let mut total: i64 = 0;

                for (user_id, _) in all_users.iter() {
                    let user = UserId::new(*user_id);
                    let user_obj = user.to_user(&ctx.http).await.unwrap();

                    let username = match user_obj.nick_in(&ctx.http, guild_id).await {
                        Some(nick) => nick,
                        None => user_obj.name,
                    };

                    let user_total = metas
                        .iter()
                        .filter(|m| m.user_id == *user_id as i64)
                        .map(|m| m.amount as i64)
                        .sum::<i64>();

                    if user_total > 0 {
                        total += user_total;
                    }

                    leaderboard_data.push((username.clone(), user_total));
                }

                leaderboard_data.sort_by(|a, b| b.1.cmp(&a.1));

                let mut completos: Vec<String> = Vec::new();
                let mut incompletos: Vec<String> = Vec::new();
                let mut sem_entrega: Vec<String> = Vec::new();

                let mut i = 1;

                for (username, value) in leaderboard_data {
                    let pos = format!("{:>2}", i);
                    let formatted_value = utils::format_amount(value as u64);
                    let username = format!("{: <20}", username);

                    if value >= current_meta {
                        completos.push(format!(
                            "+ {pos}. {username} | ✅ R$ {formatted_value} sujo"
                        ));
                    } else if value > 0 {
                        incompletos.push(format!(
                            "[2;33m! {pos}. {username} | ⚠️ R$ {formatted_value} sujo[0m"
                        ));
                    } else {
                        sem_entrega.push(format!("- {pos}. {username} | ❌ R$ 0 sujo"));
                    }

                    i += 1;
                }

                let mut message = String::from("```ini\n[📊 LEADERBOARD - Semana Atual]\n```\n\n");

                if !completos.is_empty() {
                    message.push_str("```diff\n");
                    for line in &completos {
                        message.push_str(line);
                        message.push('\n');
                    }

                    message.push_str("```\n");
                }

                if !incompletos.is_empty() {
                    message.push_str("```ansi\n");
                    for line in &incompletos {
                        message.push_str(line);
                        message.push('\n');
                    }

                    message.push_str("```\n");
                }

                if !sem_entrega.is_empty() {
                    message.push_str("```diff\n");
                    for line in &sem_entrega {
                        message.push_str(line);
                        message.push('\n');
                    }
                    message.push_str("```\n");
                }

                message.push_str(&format!(
                    "```ini\n[💰 TOTAL GERAL: R$ {} ({})]\n```",
                    utils::format_amount(total as u64),
                    total
                ));

                let builder = CreateMessage::default().content(message);
                if let Err(e) = ChannelId::new(channels.meta_channel_id)
                    .send_message(&ctx.http, builder)
                    .await
                {
                    println!("❌ Failed to send message to user channel: {:?}", e);
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
        .add(job02)
        .await
        .expect("❌ Failed to add scheduled job");

    scheduler
        .start()
        .await
        .expect("❌ Failed to start scheduler");
}

async fn notify(ctx: &Context, days: i32) -> () {
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

        let Ok(metas_for_user) = repo.get_user_approved_weekly(user_id as i64).await else {
            eprintln!("❌ - Failed to fetch approved metas.");
            return;
        };

        let total: i32 = metas_for_user.iter().map(|meta| meta.amount as i32).sum();

        if let Err(e) = send_message(
            &ctx,
            channel_id,
            user_id,
            total as i64,
            current_meta as i64,
            days,
        )
        .await
        {
            println!("❌ - Failed to send message: {:?}", e);
        }

        sleep(Duration::from_secs(1)).await;
    }
}

async fn send_message(
    ctx: &Context,
    channel_id: u64,
    user_id: u64,
    total: i64,
    current_meta: i64,
    days: i32,
) -> Result<(), serenity::Error> {
    let content = format!(
        "<@{}> Você foi **AVISADO**. Faltam **{} dias** para o fim do prazo de entrega e falta `{}` a ser entregue.",
        user_id,
        days,
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
