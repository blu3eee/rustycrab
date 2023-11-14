use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, time::Instant };

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
    },
};

pub struct PingCommand;

#[async_trait]
impl ContextCommand for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    async fn run(
        &self,
        client: &DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Example: Use command_args if needed for the logic
        // If PingCommand doesn't need arguments, this part can remain unchanged

        let start_time = Instant::now();
        let response = client
            .send_message(msg.channel_id, MessageContent::Text("Ping...".to_string())).await?
            .model().await?;
        let duration = start_time.elapsed();
        let response_time = duration.as_millis();

        client.edit_message(
            msg.channel_id,
            response.id,
            MessageContent::Text(format!("Pong!! `{} ms`", response_time))
        ).await?;

        Ok(())
    }
}
