pub mod log_settings;
pub mod action_logs;
pub mod log_ignores;

use axum::Router;

use crate::{
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
    default_router::DefaultRoutes,
};

use self::log_ignores::ignore_routes;

pub async fn bot_logs_routes() -> Router {
    let router = Router::new()
        .merge(
            <log_settings::BotGuildLogSettingsRoutes as UniqueBotGuildEntityRoutes>::router().await
        )
        .merge(action_logs::ActionLogsRoutes::router().await)
        .merge(ignore_routes().await);

    Router::new().nest("/logs", router)
}
