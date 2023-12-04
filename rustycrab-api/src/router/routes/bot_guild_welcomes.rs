use crate::unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes;
use crate::database::bot_guild_welcomes::Model as WelcomeModel;
use crate::default_router::DefaultRoutes;
use crate::queries::guild_welcome_queries::GuildWelcomeQueries;

use async_trait::async_trait;
use rustycrab_model::response::bot_guild_welcome::ResponseGuildWelcome;

pub struct BotGuildWelcomesRoutes {}

impl BotGuildWelcomesRoutes {}

#[async_trait]
impl DefaultRoutes for BotGuildWelcomesRoutes {
    type Queries = GuildWelcomeQueries;

    type ResponseJson = ResponseGuildWelcome;

    fn path() -> String {
        format!("welcomes")
    }
}

impl UniqueBotGuildEntityRoutes for BotGuildWelcomesRoutes {}

impl From<WelcomeModel> for ResponseGuildWelcome {
    fn from(model: WelcomeModel) -> Self {
        Self {
            id: model.id,
            channel_id: model.channel_id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            message_id: model.message_id,
        }
    }
}
