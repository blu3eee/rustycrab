pub mod log_settings;
pub mod action_logs;

use axum::Router;

use crate::{
    app_state::AppState,
    bot_guild_entity_router::BotGuildEntityRoutes,
    default_router::DefaultRoutes,
};

pub async fn bot_logs_routes(state: AppState) -> Router {
    let router = Router::new()
        .merge(
            <log_settings::BotGuildLogSettingsRoutes as BotGuildEntityRoutes>::router(
                state.clone()
            ).await
        )
        .merge(action_logs::ActionLogsRoutes::router(state.clone()).await);

    Router::new().nest("/logs", router)
}
