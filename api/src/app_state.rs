use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use std::collections::HashMap;
use tokio::sync::RwLock;

use std::sync::Arc;

use crate::{ BotId, discordrs::client::DiscordClient };

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub running_bots: Arc<RwLock<HashMap<BotId, DiscordClient>>>,
}
