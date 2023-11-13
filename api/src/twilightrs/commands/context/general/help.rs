use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, time::Instant };

use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        commands::context::{ ContextCommand, context_command_dispatcher::ContextCommandDispatcher },
        client::{ DiscordClient, MessageContent },
    },
};

pub struct HelpCommand;

#[async_trait]
impl ContextCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    async fn run(
        &self,
        client: &DiscordClient,
        config: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        args: &[&str]
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let dispatcher = ContextCommandDispatcher::new();
        Ok(())
    }
}
