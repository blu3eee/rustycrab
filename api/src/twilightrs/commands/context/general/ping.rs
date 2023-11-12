use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use twilight_http::Client as HttpClient;
use twilight_model::{ channel::Message, gateway::payload::incoming::MessageCreate };
use std::{ sync::Arc, error::Error, time::Instant };

use crate::{
    database::bot_guild_configurations,
    twilightrs::{ commands::context::ContextCommand, client::DiscordClient },
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
        // let reply = client
        //     .send_message(message.channel_id, MessageContent::Text("Ping...".to_string()))
        //     .unwrap();
        let response: twilight_http::Response<Message> = client.http
            .create_message(msg.channel_id)
            .content("Ping!")?
            .reply(msg.id).await?;

        let duration = start_time.elapsed();
        let response_time = duration.as_millis(); // Convert duration to milliseconds

        // let edit_content = format!("Pong! `{}` ms", response_time);
        // let _ = client.edit_message(
        //     message.channel_id,
        //     reply.id,
        //     MessageContent::Text(edit_content)
        // ).await;

        // Additional logic as needed
        println!("{:?}", response);
        Ok(())
    }

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }
}
