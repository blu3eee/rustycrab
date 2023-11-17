use crate::{
    database::log_settings::Model as LogSettingModel,
    default_router::DefaultRoutes,
    queries::guild_logs::log_setting_queries::LogSettingQueries,
    app_state::AppState,
    bot_guild_entity_router::BotGuildEntityRoutes,
};

use async_trait::async_trait;
use axum::Router;
use serde::{ Deserialize, Serialize };

pub struct BotGuildLogSettingsRoutes {}

impl BotGuildLogSettingsRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildLogSettingsRoutes {
    type Queries = LogSettingQueries;

    type ResponseJson = ResponseLogSetting;

    fn path() -> String {
        format!("settings")
    }

    async fn more_routes(_: AppState) -> Router {
        Router::new()
    }
}

impl BotGuildEntityRoutes for BotGuildLogSettingsRoutes {}

#[derive(Deserialize)]
pub struct RequestCreateLogSetting {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateLogSetting {
    pub specify_channels: Option<i8>,
    pub new_account_age: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseLogSetting {
    pub id: i32,
    pub specify_channels: i8,
    pub new_account_age: i32,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
}

impl From<LogSettingModel> for ResponseLogSetting {
    fn from(model: LogSettingModel) -> Self {
        Self {
            id: model.id,
            specify_channels: model.specify_channels,
            new_account_age: model.new_account_age,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
        }
    }
}
