use async_trait::async_trait;
use discord::model::Message;
use std::time::Instant;
use crate::{
    discordrs::{ client::DiscordClient, MessageContent, commands::context::ContextCommandHandler },
    database::bot_guild_configurations::Model as GuildConfig,
};

pub struct PingCommand;

#[async_trait]
impl ContextCommandHandler for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn category(&self) -> &'static str {
        "General Information"
    }

    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        _: &GuildConfig,
        message: &Message,
        _: &[&str]
    ) {
        let start_time = Instant::now();
        let reply = client
            .send_message(message.channel_id, MessageContent::Text("Ping...".to_string()))
            .unwrap();

        let duration = start_time.elapsed();
        let response_time = duration.as_millis(); // Convert duration to milliseconds

        let edit_content = format!("Pong! `{}` ms", response_time);
        let _ = client.edit_message(
            message.channel_id,
            reply.id,
            MessageContent::Text(edit_content)
        ).await;
    }
}
