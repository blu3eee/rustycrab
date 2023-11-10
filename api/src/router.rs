use crate::{
    routes::{
        hello_world::hello_world,
        bots::{ get_all_bots::get_all_bots, get_one_bot::get_bot_from_discord_id },
        guild_configs::get_one_config::get_one_config_by_discord_id,
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
        .route("/bots", get(get_all_bots)) // The `get_all_bots` function will be a standalone async function
        .route("/bots/:botId", get(get_bot_from_discord_id))
        .route("/guild-configs/:botId/:guildId", get(get_one_config_by_discord_id))
        .layer(Extension(app_state)) // Apply the app_state to all routes
}
