use aura::run_bot;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    if let Err(err) = run_bot().await {
        eprintln!("Error starting the bot: {err}");
    }
}