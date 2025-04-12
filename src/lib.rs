pub mod bot;
pub mod db;
pub mod scheduler;

use anyhow::Context;
use serenity::Client;
use std::env;

pub async fn run_bot() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("❌ - Database URL not found!");
    let token = env::var("DISCORD_TOKEN").expect("❌ - Token not found!");

    let db = db::init_db(&database_url)
        .await
        .context("Error Starting the database #01")?;

    println!("[INFO] - Database initialized successfully!");

    let mut client = Client::builder(token, bot::models::intents::default_itents())
        .event_handler(bot::handlers::event::Handler)
        .await
        .expect("[❌] - Error creating client!");

    {
        let mut data = client.data.write().await;
        let repo = db::models::repository::Repository::new(db);
        data.insert::<bot::models::client::ClientData>(repo);
    }

    if let Err(err) = client.start().await {
        println!("[❌] - Client error: {err:?}");
    }

    Ok(())
}
