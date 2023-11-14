use crate::{
    routes::{
        hello_world::hello_world,
        guild_configs::get_one_config::get_one_config_by_discord_id,
        bots,
    },
    app_state::AppState,
};

use axum::{ routing::get, Router, Extension };

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .route("/hello", get(hello_world))
        .nest("/bots", bots::bots_router())
        .route("/guild-configs/:botId/:guildId", get(get_one_config_by_discord_id))
        .layer(Extension(app_state)) // Apply the app_state to all routes
}
