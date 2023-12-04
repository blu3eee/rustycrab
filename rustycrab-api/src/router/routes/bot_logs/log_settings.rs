use crate::{
    database::log_settings::Model as LogSettingModel,
    default_router::DefaultRoutes,
    queries::guild_logs::log_setting_queries::LogSettingQueries,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
};

use async_trait::async_trait;
use rustycrab_model::response::logs::setting::ResponseLogSetting;

pub struct BotGuildLogSettingsRoutes {}

impl BotGuildLogSettingsRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildLogSettingsRoutes {
    type Queries = LogSettingQueries;

    type ResponseJson = ResponseLogSetting;

    fn path() -> String {
        format!("settings")
    }
}

impl UniqueBotGuildEntityRoutes for BotGuildLogSettingsRoutes {}

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
