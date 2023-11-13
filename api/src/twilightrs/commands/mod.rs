pub mod context;
pub mod slash;

use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, sync::Arc };

use crate::database::bot_guild_configurations;

use super::{ client::DiscordClient, dispatchers::ClientDispatchers };

pub async fn context_commands_handler(
    client: &DiscordClient,
    dispatchers: Arc<ClientDispatchers>,
    config: &bot_guild_configurations::Model,
    msg: &MessageCreate,
    command_name: &str,
    command_args: &[&str]
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // match cmd_name {
    //     "ping" => {}
    //     _ => {}
    // }
    println!("context_commands_handler");
    dispatchers.context_commands.dispatch_command(
        client,
        config,
        msg,
        command_name,
        command_args
    ).await;
    Ok(())
}
