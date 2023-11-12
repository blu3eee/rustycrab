pub mod context_command_dispatcher;
pub mod general;

use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::client::DiscordClient,
};

use async_trait::async_trait;

#[async_trait]
pub trait ContextCommand: Send + Sync {
    fn name(&self) -> &'static str;

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    async fn run(
        &self,
        client: &DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        cmd_args: &[&str]
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub trait ContextCommandCategory {
    fn name(&self) -> &'static str;
    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>>;
}

pub struct ContextCommandHandler {
    pub command_name: &'static str,
    pub category_name: &'static str,
    pub command: Box<dyn ContextCommand>,
}
