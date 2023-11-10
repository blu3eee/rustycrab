use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    discordrs::client::DiscordClient,
};
use discord::model::Message;

use self::context::CommandDispatcher;

pub mod context;

// In your process_commands function:
pub async fn process_commands(client: &mut DiscordClient, config: &GuildConfig, message: &Message) {
    let content = message.content.trim();
    if let Some(stripped) = content.strip_prefix(&config.prefix) {
        let command = stripped.trim_start_matches(' '); // Remove leading spaces after the prefix
        let parts: Vec<&str> = command.split_whitespace().collect();
        if let Some((&command_name, args)) = parts.split_first() {
            let command_dispatcher = CommandDispatcher::new();
            command_dispatcher.dispatch_command(client, command_name, config, message, args).await;
        }
    }
}
