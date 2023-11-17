use async_trait::async_trait;
use axum::Router;
use serde::{ Deserialize, Serialize };

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    default_router::DefaultRoutes,
    queries::guild_config_queries::GuildConfigQueries,
    bot_guild_entity_router::BotGuildEntityRoutes,
    app_state::AppState,
};

use super::{ bots::ResponseBot, guilds::ResponseGuild };

pub struct BotGuildConfigsRoutes {}

impl BotGuildConfigsRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildConfigsRoutes {
    type Queries = GuildConfigQueries;

    type ResponseJson = ResponseGuildConfig;

    fn path() -> String {
        format!("configs")
    }

    async fn more_routes(_: AppState) -> Router {
        Router::new()
    }
}

impl BotGuildEntityRoutes for BotGuildConfigsRoutes {}

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
    pub bot: Option<ResponseBot>,
    pub guild_id: Option<ResponseGuild>,
    pub module_flags: i32,
    pub premium_flags: i32,
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
