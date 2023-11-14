pub mod context;
pub mod slash;

use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, sync::Arc };

use crate::database::bot_guild_configurations;

use self::context::context_command_dispatcher::ContextCommandDispatcher;

use super::{ discord_client::DiscordClient, dispatchers::ClientDispatchers };

pub async fn context_commands_handler(
    client: &DiscordClient,
    config: &bot_guild_configurations::Model,
    msg: &MessageCreate,
    command_name: &str,
    command_args: &[&str]
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // match cmd_name {
    //     "ping" => {}
    //     _ => {}
    // }
    let dispatcher = ContextCommandDispatcher::new();
    dispatcher.dispatch_command(client, config, msg, command_name, command_args).await;
    Ok(())
}
