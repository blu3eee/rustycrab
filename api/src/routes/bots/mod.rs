// routes/bot/mod.rs
mod get_all_bots;
mod get_one_bot;
use serde::{ Deserialize, Serialize };

use crate::database::bots::Model as BotModel;

use axum::{ Router, routing::get };

pub fn bots_router() -> Router {
    Router::new()
        .route("/", get(get_all_bots::get_all_bots))
        .route("/:botId", get(get_one_bot::get_bot_from_discord_id))
}

#[derive(Deserialize)]
pub struct RequestCreateBot {
    pub bot_id: String,
    pub token: String,
    pub theme_hex_color: Option<String>,
    pub discord_secret: Option<String>,
    pub discord_callback_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
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

impl From<BotModel> for ResponseBot {
    fn from(model: BotModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            token: model.token,
            theme_hex_color: model.theme_hex_color,
            discord_callback_url: model.discord_callback_url,
            discord_secret: model.discord_secret,
            premium_flags: model.premium_flags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBot {
    pub data: ResponseBot,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBots {
    pub data: Vec<ResponseBot>,
}
