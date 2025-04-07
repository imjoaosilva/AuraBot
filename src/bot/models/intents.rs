use serenity::all::GatewayIntents;

pub fn default_itents() -> GatewayIntents {
    GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
}