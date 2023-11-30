use serde::{ Deserialize, Serialize };

use super::{ bots::ResponseBot, guilds::ResponseGuild };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateConfig {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestUpdateConfig {
    pub bot_id: i32,
    pub guild_id: i32,
    pub prefix: Option<String>,
    pub locale: Option<String>,
    pub module_flags: Option<i32>,
    pub premium_flags: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseGuildConfig {
    pub id: i32,
    pub prefix: String,
    pub locale: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub module_flags: i32,
    pub premium_flags: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseGuildConfigDetails {
    pub id: i32,
    pub prefix: String,
    pub locale: String,
    pub bot: Option<ResponseBot>,
    pub guild_id: Option<ResponseGuild>,
    pub module_flags: i32,
    pub premium_flags: i32,
}
