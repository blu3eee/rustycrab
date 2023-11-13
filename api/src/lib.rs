pub mod app_state;
pub mod router;
pub mod routes;
pub mod database;
pub mod utilities;
pub mod queries;
pub mod twilightrs;

use app_state::AppState;
use axum::Router;
use queries::bot_queries;
use sea_orm::DatabaseConnection;

use twilight_cache_inmemory::{ ResourceType, InMemoryCache };
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{ MinimalActivity, ActivityType, Status },
};
use twilightrs::client::DiscordClient;
use twilightrs::events::handle_bot_events;
use utilities::app_error::AppError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::router::create_router;
use crate::database::bots::Model as BotModel;

// discord
use twilight_gateway::{ Intents, ShardId, Shard, Config };
use twilight_http::Client as HttpClient;

#[macro_export]
macro_rules! cdn_avatar {
    // https://cdn.discordapp.com/avatars/{user}/{avatar}.jpg
    ($user:expr, $avatar:expr) => {
        format!("https://cdn.discordapp.com/avatars/{}/{}.jpg?size=4096", $user, $avatar)
    };
}

#[macro_export]
macro_rules! cdn_emoji {
    ($emoji_id:expr) => {
        format!("https://cdn.discordapp.com/emojis/{}.png?size=4096", $emoji_id)
    };
}

pub async fn run(app_state: AppState) {
    let app: Router = Router::new().nest("/api", create_router(app_state).await);

    println!("Starting server on 127.0.0.1:8080");
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

pub async fn running_bots(
    db: &DatabaseConnection
) -> Result<HashMap<String, DiscordClient>, AppError> {
    let bots: Vec<BotModel> = bot_queries::get_all_bots(&db).await?;
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
        let cache = InMemoryCache::builder().resource_types(ResourceType::all()).build();

        // Only HTTP client is stored in DiscordClient
        let client = DiscordClient {
            db: db.clone(),
            http: http.clone(),
        };

        discord_clients.insert(bot.bot_id, client.clone());

        // Handle events with the shard in a separate task
        tokio::spawn(async move { handle_bot_events(shard, cache, client).await });
    }

    Ok(discord_clients)
}
