use crate::{
    router::routes::{
        hello_world::hello_world,
        bots::BotsRouter,
        bot_guild_configs::BotGuildConfigsRoutes,
        bot_guild_welcomes::BotGuildWelcomesRoutes,
        bot_logs::bot_logs_routes,
    },
    app_state::AppState,
    default_router::DefaultRoutes,
    bot_guild_entity_router::BotGuildEntityRoutes,
};

use axum::{ routing::get, Router, Extension };

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .route("/hello", get(hello_world))
        // .nest("/bots", bots::bots_router())
        .merge(BotsRouter::router(app_state.clone()).await)
        .merge(<BotGuildConfigsRoutes as BotGuildEntityRoutes>::router(app_state.clone()).await)
        .merge(<BotGuildWelcomesRoutes as BotGuildEntityRoutes>::router(app_state.clone()).await)
        .merge(bot_logs_routes(app_state.clone()).await)
        .layer(Extension(app_state)) // Apply the app_state to all routes
}
