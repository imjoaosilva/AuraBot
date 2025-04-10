use serenity::all::{Context, CreateEmbed, CreateMessage, Timestamp};

pub async fn send_log(ctx: &Context, message: String, logs_channel_id: u64) {

    let logs_channel = serenity::all::ChannelId::new(logs_channel_id);

    let embed = CreateEmbed::default()
        .description(message)
        .timestamp(Timestamp::now())
        .colour(serenity::all::Colour::LIGHT_GREY);

    let builder = CreateMessage::default().add_embed(embed);

    logs_channel.send_message(&ctx.http, builder).await.unwrap();
}
