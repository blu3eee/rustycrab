// routes/bot_guild_configurations/mod.rs
pub mod get_one_config;
pub mod create_config_extractor;
pub mod create_config;
use serde::{ Deserialize, Serialize };

use crate::database::bot_guild_configurations::Model as GuildConfig;

use super::bots::ResponseBot;

#[derive(Deserialize)]
pub struct RequestCreateConfig {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateConfig {
    pub prefix: Option<String>,
    pub locale: Option<String>,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
    pub module_flags: Option<i32>,
    pub premium_flags: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseGuildConfig {
    pub id: i32,
    pub prefix: String,
    pub locale: String,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
    pub module_flags: i32,
    pub premium_flags: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseGuildConfigWithBotInfo {
    pub id: i32,
    pub prefix: String,
    pub locale: String,
    pub bot: ResponseBot,
    pub guild_id: Option<i32>,
    pub module_flags: i32,
    pub premium_flags: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataGuildConfig {
    pub data: ResponseGuildConfig,
}

impl From<GuildConfig> for ResponseGuildConfig {
    fn from(model: GuildConfig) -> Self {
        Self {
            id: model.id,
            prefix: model.prefix,
            locale: model.locale,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            module_flags: model.module_flags,
            premium_flags: model.premium_flags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataGuildConfigs {
    pub data: Vec<ResponseGuildConfig>,
}

// Function to convert from SeaORM Model to you
