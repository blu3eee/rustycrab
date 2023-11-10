use std::net::SocketAddr;

use axum::Router;
use discordrs::client::DiscordClient;
use queries::bot_queries;
use router::create_router;

// local modules
pub mod routes;
pub mod router;
pub mod database;
pub mod utilities;
pub mod queries;
pub mod app_state;
pub mod discordrs;
// use crate::{ database::bots::{ self, Model as BotModel } };
use app_state::AppState;
use sea_orm::DatabaseConnection;
use database::bots::Model as BotModel;
use utilities::app_error::AppError;

// discord
use discord::{ Discord, Connection, State, model::ReadyEvent };

use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub async fn run(app_state: AppState) {
    let app: Router = Router::new().nest("/api", create_router(app_state).await);

    println!("Starting server on 127.0.0.1:8080");
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

pub type BotId = String; // Replace with the actual type of your bot ID

pub async fn run_discord_bots(
    db: &DatabaseConnection
) -> Result<Arc<RwLock<HashMap<BotId, DiscordClient>>>, AppError> {
    let bots: Vec<BotModel> = bot_queries::get_all_bots(&db).await?;
    let running_clients: Arc<RwLock<HashMap<BotId, DiscordClient>>> = Arc::new(
        RwLock::new(HashMap::new())
    );

    for bot in bots {
        let running_clients_cloned: Arc<
            RwLock<HashMap<BotId, DiscordClient>>
        > = running_clients.clone();
        let token = bot.token.clone(); // Clone the token to move into the async block
        let bot_id = bot.bot_id.clone();
        let db_clone = db.clone(); // Clone the database connection as needed

        tokio::spawn(async move {
            match start_bot(&token).await {
                Ok((discord, connection, ready)) => {
                    let state = State::new(ready);
                    let client: DiscordClient = DiscordClient {
                        db: db_clone,
                        bot_id: bot_id,
                        discord: discord,
                        connection: connection,
                        state: state,
                    };
                    let mut clients_map = running_clients_cloned.write().await;
                    clients_map.insert(bot.bot_id.clone(), client);
                    drop(clients_map); // Drop the write lock before spawning a new task
                    listen_to_events(bot.bot_id, running_clients_cloned.clone()).await;
                }
                // Handle errors as before
                Err(e) => {
                    eprintln!("Error starting bot with ID {}: {:?}", bot.bot_id, e);
                }
            }
        });
    }

    Ok(running_clients)
}

async fn start_bot(token: &str) -> Result<(Discord, Connection, ReadyEvent), discord::Error> {
    // Log in to Discord using the bot token
    let discord = Discord::from_bot_token(token)?;

    // Establish and maintain a websocket connection
    let (connection, ready_event) = discord.connect()?;
    println!(
        "[{}#{}] successfully connected.",
        &ready_event.user.username,
        format!("{:04}", &ready_event.user.discriminator)
    );

    Ok((discord, connection, ready_event))
}

// This function now takes a DiscordClient and processes events.
async fn listen_to_events(
    bot_id: BotId,
    running_clients: Arc<RwLock<HashMap<BotId, DiscordClient>>>
) {
    if let Some(client) = running_clients.write().await.get_mut(&bot_id) {
        loop {
            match client.connection.recv_event() {
                Ok(event) => {
                    client.state.update(&event);
                    client.process_events(&event).await;
                }
                // Handle other cases as before
                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    break;
                }
                Err(err) => {
                    println!("Receive error: {:?}", err);
                    break;
                }
            }
        }
    }
}
