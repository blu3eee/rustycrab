use async_trait::async_trait;
use rustycrab_model::response::bot_guild_config::ResponseGuildConfig;

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    default_router::DefaultRoutes,
    queries::guild_config_queries::GuildConfigQueries,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
};

pub struct BotGuildConfigsRoutes {}

impl BotGuildConfigsRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildConfigsRoutes {
    type Queries = GuildConfigQueries;

    type ResponseJson = ResponseGuildConfig;

    fn path() -> String {
        format!("configs")
    }
}

impl UniqueBotGuildEntityRoutes for BotGuildConfigsRoutes {}

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
