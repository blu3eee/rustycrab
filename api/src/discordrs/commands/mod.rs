use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    discordrs::client::DiscordClient,
};
use discord::model::Message;

use self::context::dispatcher::ContextCommandDispatcher;

pub mod context;

// In your process_commands function:
pub async fn process_context_commands(
    client: &mut DiscordClient,
    config: &GuildConfigModel,
    message: &Message,
    command_dispatcher: &ContextCommandDispatcher
) {
    let content = message.content.trim();
    if let Some(stripped) = content.strip_prefix(&config.prefix) {
        let parts: Vec<&str> = stripped.trim_start_matches(' ').split_whitespace().collect();
        if let Some((&command_name, args)) = parts.split_first() {
            // Borrow the dispatcher mutably
            command_dispatcher.dispatch_command(client, command_name, config, message, args).await;
        }
    }
}
