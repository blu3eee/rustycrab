use serde::{ Serialize, Deserialize };
use super::{
    discord_message::RequestCreateUpdateMessage,
    discord_message::ResponseMessageDetails,
    guilds::ResponseGuild,
    bots::ResponseBot,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAutoRes {
    pub id: i32,
    pub trigger: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub response_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAutoResDetails {
    pub id: i32,
    pub trigger: String,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub response: Option<ResponseMessageDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateAutoResponse {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub trigger: String,
    pub response_data: RequestCreateUpdateMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestUpdateAutoResponse {
    pub trigger: Option<String>,
    pub response_data: Option<RequestCreateUpdateMessage>,
}
