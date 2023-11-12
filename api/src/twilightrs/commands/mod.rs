pub mod context;
pub mod slash;

use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::database::bot_guild_configurations;

use super::client::DiscordClient;

pub async fn context_commands_handler(
    client: &DiscordClient,
    config: &bot_guild_configurations::Model,
    msg: &MessageCreate,
    cmd_name: &str,
    cmd_args: &[&str]
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cmd_name {
        "ping" => {}
        _ => {}
    }
    Ok(())
}
