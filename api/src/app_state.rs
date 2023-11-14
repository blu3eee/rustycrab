use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use std::{ collections::HashMap, sync::Arc };

use crate::twilightrs::discord_client::DiscordClient;

// use crate::BotId;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub running_bots: HashMap<String, Arc<DiscordClient>>,
}
