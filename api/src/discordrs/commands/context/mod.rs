mod general;
pub mod dispatcher;

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    discordrs::client::DiscordClient,
};

use discord::model::Message;
use async_trait::async_trait;

#[async_trait]
pub trait ContextCommand: Send + Sync {
    fn name(&self) -> &'static str;

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        config: &GuildConfig,
        message: &Message,
        args: &[&str]
    );
}

pub trait ContextCommandCategory {
    fn name(&self) -> &'static str;
    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>>;
}

struct ContextCommandHandler {
    command_name: &'static str,
    category_name: &'static str,
    command: Box<dyn ContextCommand>,
}
