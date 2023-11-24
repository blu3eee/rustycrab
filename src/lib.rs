//! Main module for the Discord bot application.
//!
//! This module sets up and runs the application, initializing all necessary components like
//! the Axum web server, database connections, and Discord bot clients. It also defines several
//! utility macros for constructing Discord CDN URLs.

pub mod app_state;
pub mod router;
pub mod database;
pub mod utilities;
pub mod queries;
pub mod twilightrs;
pub mod locales;
pub mod default_queries;
pub mod default_router;
pub mod unique_bot_guild_entity_queries;
pub mod unique_bot_guild_entity_router;
pub mod multi_bot_guild_entities_queries;
pub mod multi_bot_guild_entities_router;

use app_state::AppState;
use axum::Router;
use default_queries::DefaultSeaQueries;
use queries::bot_queries::BotQueries;
use sea_orm::DatabaseConnection;

use utilities::app_error::AppError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::router::create_router::create_router;
use crate::database::bots::Model as BotModel;

// discord
use twilight_gateway::{ Intents, ShardId, Shard, Config };
use twilight_http::Client as HttpClient;
use twilight_cache_inmemory::{ ResourceType, InMemoryCache };
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{ MinimalActivity, ActivityType, Status },
};
use twilightrs::discord_client::DiscordClient;
use twilightrs::events::handle_bot_events;

/// Creates a URL to a user's avatar on Discord's CDN.
#[macro_export]
macro_rules! cdn_avatar {
    // https://cdn.discordapp.com/avatars/{user}/{avatar}.jpg
    ($user_id:expr, $avatar_hash:expr) => {
        format!("https://cdn.discordapp.com/avatars/{}/{}.jpg?size=4096", $user_id, $avatar_hash)
    };
}

/// Creates a URL to a Discord emoji on Discord's CDN.
#[macro_export]
macro_rules! cdn_emoji {
    ($emoji_id:expr) => {
        format!("https://cdn.discordapp.com/emojis/{}.png?size=4096", $emoji_id)
    };
}

/// Creates a URL to a guild's icon on Discord's CDN.
#[macro_export]
macro_rules! cdn_guild_icon {
    // https://cdn.discordapp.com/avatars/{user}/{avatar}.jpg
    ($guild_id:expr, $icon_hash:expr) => {
        format!("https://cdn.discordapp.com/icons/{}/{}.png?size=4096", $guild_id, $icon_hash)
    };
}

/// Starts the Axum web server and sets up routing.
///
/// This function initializes the Axum router with the provided application state,
/// then binds and serves the application on a specified address.
pub async fn run(app_state: AppState) {
    let app: Router = Router::new().nest("/api", create_router(app_state).await);

    println!("Starting server on 127.0.0.1:8080");
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

/// Initializes and runs all bots present in the database.
///
/// For each bot found in the database, this function sets up the necessary Discord client,
/// including its configuration, HTTP client, and event handling.
pub async fn running_bots(
    db: &DatabaseConnection
) -> Result<HashMap<String, Arc<DiscordClient>>, AppError> {
    let bots: Vec<BotModel> = BotQueries::find_all(&db).await?;

    let mut discord_clients = HashMap::new();
    for bot in bots {
        let config = Config::builder(bot.token.clone(), Intents::all())
            .presence(
                UpdatePresencePayload::new(
                    vec![
                        (MinimalActivity {
                            kind: ActivityType::Playing,
                            name: "Rusty Crab".into(),
                            url: None,
                        }).into()
                    ],
                    false,
                    None,
                    Status::Idle
                ).map_err(|e|
                    AppError::internal_server_error(
                        format!("Error creating presence for bot {:?}", e)
                    )
                )?
            )
            .build();
        let shard = Shard::with_config(ShardId::ONE, config);
        let http = Arc::new(HttpClient::new(bot.token.clone()));
        let cache: Arc<InMemoryCache> = Arc::new(
            InMemoryCache::builder().resource_types(ResourceType::all()).build()
        );
        // Only HTTP client is stored in DiscordClient
        let client = Arc::new(DiscordClient::new(db.clone(), http.clone(), cache.clone()));

        discord_clients.insert(bot.bot_id, client.clone());

        // Handle events with the shard in a separate task
        tokio::spawn(async move { handle_bot_events(shard, client).await });
    }

    Ok(discord_clients)
}
