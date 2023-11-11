mod general;
mod music;

use std::collections::HashMap;

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    discordrs::client::DiscordClient,
};

use discord::model::Message;
use async_trait::async_trait;

use self::general::ping::PingCommand;
// use struct_iterable::Iterable;

#[async_trait]
pub trait ContextCommandHandler: Send + Sync {
    fn name(&self) -> &'static str;

    fn category(&self) -> &'static str;

    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        config: &GuildConfig,
        message: &Message,
        args: &[&str]
    );
}

pub struct ContextCommandDispatcher {
    commands: HashMap<String, Box<dyn ContextCommandHandler>>,
}

impl ContextCommandDispatcher {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn ContextCommandHandler>> = HashMap::new();

        // Add commands to the dispatcher
        commands.insert("ping".to_string(), Box::new(PingCommand {}));
        // ... other commands ...

        ContextCommandDispatcher { commands }
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
