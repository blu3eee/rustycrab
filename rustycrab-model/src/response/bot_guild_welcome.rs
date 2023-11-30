use serde::{ Deserialize, Serialize };
use super::bots::ResponseBot;

use super::discord_message::{ RequestCreateUpdateMessage, ResponseMessageDetails };
use super::guilds::ResponseGuild;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateWelcome {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub message_data: Option<RequestCreateUpdateMessage>,
    pub channel_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateWelcome {
    pub channel_id: Option<String>,
    pub message_data: Option<RequestCreateUpdateMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseGuildWelcome {
    pub id: i32,
    pub enabled: i8,
    pub channel_id: Option<String>,
    pub bot_id: i32,
    pub guild_id: i32,
    pub message_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseGuildWelcomeDetails {
    pub id: i32,
    pub enabled: i8,
    pub channel_id: Option<String>,
    pub bot: Option<ResponseBot>,
    pub guild: Option<ResponseGuild>,
    pub message: Option<ResponseMessageDetails>,
}
