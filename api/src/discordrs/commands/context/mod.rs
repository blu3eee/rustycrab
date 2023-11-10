mod music;
mod ping;

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    discordrs::client::DiscordClient,
};
use std::collections::HashMap;
use discord::model::Message;
use async_trait::async_trait;

use self::ping::PingCommand;

#[async_trait]
trait CommandHandler: Send + Sync {
    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        config: &GuildConfig,
        message: &Message,
        args: &[&str]
    );
}

pub struct CommandDispatcher {
    commands: HashMap<&'static str, Box<dyn CommandHandler>>,
}

impl CommandDispatcher {
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Box<dyn CommandHandler>> = HashMap::new();
        commands.insert("ping", Box::new(PingCommand));
        // ... Add other commands here

        CommandDispatcher { commands }
    }

    pub async fn dispatch_command(
        &self,
        client: &mut DiscordClient,
        command_name: &str,
        config: &GuildConfig,
        message: &Message,
        args: &[&str]
    ) {
        if let Some(handler) = self.commands.get(command_name) {
            handler.handle_command(client, config, message, args).await;
        } else {
            // Handle unknown command
        }
    }
}
