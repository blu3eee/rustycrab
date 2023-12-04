use crate::{
    router::routes::{
        bots::BotsRouter,
        bot_guild_configs::BotGuildConfigsRoutes,
        bot_guild_welcomes::BotGuildWelcomesRoutes,
        bot_logs::bot_logs_routes,
    },
    app_state::AppState,
    default_router::DefaultRoutes,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
};

use axum::{ routing::get, Router, Extension, middleware };

use super::{
    middlewares::log_route::log_route,
    routes::{ tickets::ticket_routes, discord_oauth::auth_routes },
};

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .merge(BotsRouter::router().await)
        .merge(<BotGuildConfigsRoutes as UniqueBotGuildEntityRoutes>::router().await)
        .merge(<BotGuildWelcomesRoutes as UniqueBotGuildEntityRoutes>::router().await)
        .merge(bot_logs_routes().await)
        .merge(ticket_routes().await)
        .layer(Extension(app_state.clone()))
        .merge(auth_routes().await)
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .layer(middleware::from_fn(log_route))
}
