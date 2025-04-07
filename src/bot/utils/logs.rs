use serenity::all::{Context, CreateEmbed, CreateMessage, Timestamp};
use std::env;

pub async fn send_log(ctx: &Context, message: String) {
    let logs_id: u64 = env::var("LOGS_CHANNEL_ID")
        .expect("❌ - Logs ID not found!")
        .parse()
        .expect("❌ - LOgs ID must be an integer");

    let logs_channel = serenity::all::ChannelId::new(logs_id);

    let embed = CreateEmbed::default()
        .description(message)
        .timestamp(Timestamp::now())
        .colour(serenity::all::Colour::LIGHT_GREY);

    let builder = CreateMessage::default().add_embed(embed);

    logs_channel.send_message(&ctx.http, builder).await.unwrap();
}
