use crate::app_state::AppState;
use crate::unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes;
use crate::database::bot_guild_welcomes::Model as WelcomeModel;
use crate::default_router::DefaultRoutes;
use crate::queries::guild_welcome_queries::GuildWelcomeQueries;

use async_trait::async_trait;
use axum::Router;
use serde::{ Deserialize, Serialize };
use super::bots::ResponseBot;

use super::guilds::ResponseGuild;
use super::{ RequestCreateUpdateMessage, ResponseMessageDetails };

pub struct BotGuildWelcomesRoutes {}

impl BotGuildWelcomesRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildWelcomesRoutes {
    type Queries = GuildWelcomeQueries;

    type ResponseJson = ResponseGuildWelcome;

    fn path() -> String {
        format!("welcomes")
    }

    async fn more_routes(_: AppState) -> Router {
        Router::new()
    }
}

impl UniqueBotGuildEntityRoutes for BotGuildWelcomesRoutes {}

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
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
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

impl From<WelcomeModel> for ResponseGuildWelcome {
    fn from(model: WelcomeModel) -> Self {
        Self {
            id: model.id,
            enabled: model.enabled,
            channel_id: model.channel_id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            message_id: model.message_id,
        }
    }
}
