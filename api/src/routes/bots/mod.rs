// routes/bot/mod.rs
pub mod get_all_bots;
pub mod get_one_bot;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct RequestCreateBot {
    pub bot_id: String,
    pub token: String,
    pub theme_hex_color: Option<String>,
    pub discord_secret: Option<String>,
    pub discord_callback_url: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestUpdateBot {
    pub bot_id: Option<String>,
    pub token: Option<String>,
    pub theme_hex_color: Option<String>,
    pub discord_secret: Option<String>,
    pub discord_callback_url: Option<String>,
    pub premium_flags: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBot {
    pub id: i32,
    pub bot_id: String,
    pub token: String,
    pub theme_hex_color: String,
    pub discord_secret: String,
    pub discord_callback_url: String,
    pub premium_flags: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBot {
    pub data: ResponseBot,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBots {
    pub data: Vec<ResponseBot>,
}
