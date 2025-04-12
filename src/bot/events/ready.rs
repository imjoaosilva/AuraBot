use super::{handlers::command::register_commands, scheduler};
use serenity::all::{Context, GuildId};
use serenity::model::application::Command;
use std::env;

pub async fn run(ctx: Context, ready: serenity::all::Ready) {
    let guild_id = env::var("GUILD_ID")
        .expect("❌ - Guild ID not found!")
        .parse()
        .expect("❌ - Guild ID must be an integer");

    let guild = GuildId::new(guild_id);

    match guild.set_commands(&ctx.http, register_commands()).await {
        Ok(list) => println!("☑️  - {} Commands loaded!", list.len()),
        Err(_) => println!("❌ - Unable to load commands!"),
    };

    match Command::set_global_commands(&ctx.http, vec![]).await {
        Ok(list) => println!("☑️  - {} Global  Commands loaded!", list.len()),
        Err(_) => println!("❌ - Unable to load commands!"),
    };

    println!("✅ - {} started successfully!", ready.user.name);

    scheduler::reminder::setup_cron_jobs(ctx.clone().into()).await;
}
