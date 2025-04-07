use serenity::all::{Context, GuildId};
use std::env;
use super::handlers::command::register_commands;

pub async fn run(ctx: Context, ready: serenity::all::Ready) {
    let guild_id = env::var("GUILD_ID")
        .expect("❌ - Guild ID not found!")
        .parse()
        .expect("❌ - Guild ID must be an integer");

    let guild = GuildId::new(guild_id);

    match guild
        .set_commands(&ctx.http, register_commands())
        .await
    {
        Ok(list) => println!("☑️  - {} Commands loaded!", list.len()),
        Err(_) => println!("❌ - Unable to load commands!"),
    };


    println!("✅ - {} started successfully!", ready.user.name);
}
