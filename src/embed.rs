use serenity::all::{Colour, CreateEmbed};
use crate::Config;

pub fn start_embed(config: &Config) -> CreateEmbed {
    CreateEmbed::default()
        .title("🎉 Server is online!")
        .description(&config.startup_message)
        .colour(Colour::from_rgb(0, 255, 0))
}

pub fn stop_embed(config: &Config) -> CreateEmbed {
    CreateEmbed::default()
        .title("⛔ Server is offline!")
        .description(&config.shutdown_message)
        .colour(Colour::from_rgb(255, 0, 0))
}