use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateBot {
    pub bot_id: String,
    pub token: String,
    pub theme_hex_color: Option<String>,
    pub discord_secret: Option<String>,
    pub discord_callback_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateBot {
    pub bot_id: Option<String>,
    pub token: Option<String>,
    pub theme_hex_color: Option<String>,
    pub discord_secret: Option<String>,
    pub discord_callback_url: Option<String>,
    pub premium_flags: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBot {
    pub id: i32,
    pub bot_id: String,
    pub token: String,
    pub theme_hex_color: String,
    pub discord_secret: String,
    pub discord_callback_url: String,
    pub premium_flags: i32,
}
