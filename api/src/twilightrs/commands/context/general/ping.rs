use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, time::Instant };

use crate::{
    database::bot_guild_configurations,
    twilightrs::{ commands::context::ContextCommand, client::{ DiscordClient, MessageContent } },
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
        _: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        _: &[&str]
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let start_time = Instant::now();

        let response = client
            .send_message(msg.channel_id, MessageContent::Text("Ping...".to_string())).await?
            .model().await?;

        let duration = start_time.elapsed();
        let response_time = duration.as_millis(); // Convert duration to milliseconds

        let _ = client.edit_message(
            msg.channel_id,
            response.id,
            MessageContent::Text(format!("Pong!! `{} ms`", response_time))
        ).await;

        Ok(())
    }
}
