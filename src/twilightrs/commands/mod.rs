pub mod context;
pub mod slash;

use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, sync::Arc };

use crate::database::bot_guild_configurations;

use super::{ discord_client::DiscordClient, dispatchers::ClientDispatchers };

pub async fn context_commands_handler(
    client: DiscordClient,
    config: &bot_guild_configurations::Model,
    dispatchers: &Arc<ClientDispatchers>,
    msg: &MessageCreate,
    command_name: &str,
    command_args: &[&str]
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dispatchers.context_commands.dispatch_command(
        client,
        config,
        msg,
        command_name,
        command_args
    ).await;
    Ok(())
}
