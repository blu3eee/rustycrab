pub mod user;
pub mod bots;
pub mod guilds;
pub mod bot_guild_config;
pub mod bot_users;
pub mod bot_guild_welcome;
pub mod logs;
pub mod ticket;
pub mod auto_response;
pub mod discord_message;
pub mod marriages;
pub mod items;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataJson<T> where T: Serialize {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataList<T> where T: Serialize {
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataMessage {
    pub message: String,
}
