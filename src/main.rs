mod embed;

use std::env;
use std::sync::Arc;
use serenity::all::ChannelId;
use serenity::builder::CreateMessage;
use serenity::http::Http;

#[derive(serde::Deserialize)]
struct Config {
    channel_id: String,
    mc_server_path: String,
    startup_message: String,
    shutdown_message: String,
}
#[tokio::main]
async fn main() {

    // Load .env
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    //Load conf.json
    let config: Config = serde_json::from_str(
        &std::fs::read_to_string("config.json").expect("config.json not found")
    ).expect("Failed to parse config.json");

    // Create an instance of the HTTP client
    let http = Arc::new(Http::new(&token));

    // Create channel and send message in it
    let channel_id = config.channel_id.parse::<u64>().expect("Invalid channel ID");
    let channel = ChannelId::from(channel_id);

    let embed = embed::start_embed(&config);
    let _ = channel.send_message(&*http, CreateMessage::default().embed(embed)).await;

    // Stop message On Ctrl+c
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl_c");

    let embed = embed::stop_embed(&config);
    let _ = channel.send_message(&*http, CreateMessage::default().embed(embed)).await;


}
