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
    unique_bot_guild_entity_router::BotGuildEntityRoutes,
};

use axum::{ routing::get, Router, Extension, middleware };

use super::{ middlewares::log_route::log_route, routes::tickets::ticket_routes };

pub async fn create_router(app_state: AppState) -> Router {
    let router = Router::new()
        .merge(BotsRouter::router(app_state.clone()).await)
        .merge(<BotGuildConfigsRoutes as BotGuildEntityRoutes>::router(app_state.clone()).await)
        .merge(<BotGuildWelcomesRoutes as BotGuildEntityRoutes>::router(app_state.clone()).await)
        .merge(bot_logs_routes(app_state.clone()).await)
        .merge(ticket_routes(app_state.clone()).await)
        .layer(Extension(app_state))
        .layer(middleware::from_fn(log_route))
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .route("/hello", get(hello_world));
    router
}
