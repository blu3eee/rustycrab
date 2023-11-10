use api::{ run, app_state::AppState, run_discord_bots, BotId, discordrs::client::DiscordClient };

use dotenv::dotenv;
use sea_orm::Database;
use std::env;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    // Placeholder for establishing database connection
    // Retrieve the database URL from environment variables
    let database_url: String = env
        ::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    // Establish a database connection
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            log::error!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    // Retrieve all bots from the database and their running state
    let running_bots: Arc<RwLock<HashMap<BotId, DiscordClient>>> = run_discord_bots(
        &db
    ).await.expect("Failed to run Discord bots");

    let app_state = AppState {
        db,
        running_bots,
    };

    run(app_state).await;

    // Run Discord bots
    //    run_discord_bots(bots).await;
}
