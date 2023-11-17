pub mod log_settings;
pub mod action_logs;
pub mod log_ignores;

use axum::Router;

use crate::{
    app_state::AppState,
    bot_guild_entity_router::BotGuildEntityRoutes,
    default_router::DefaultRoutes,
};

use self::log_ignores::ignore_routes;

pub async fn bot_logs_routes(state: AppState) -> Router {
    let router = Router::new()
        .merge(
            <log_settings::BotGuildLogSettingsRoutes as BotGuildEntityRoutes>::router(
                state.clone()
            ).await
        )
        .merge(action_logs::ActionLogsRoutes::router(state.clone()).await)
        .merge(ignore_routes(state.clone()).await);

    Router::new().nest("/logs", router)
}
