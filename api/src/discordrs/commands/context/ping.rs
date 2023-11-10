use async_trait::async_trait;
use discord::model::Message;

use crate::{
    discordrs::{ client::DiscordClient, MessageContent, DiscordEmbed },
    database::bot_guild_configurations::Model as GuildConfig,
};

use super::CommandHandler;

pub struct PingCommand;

#[async_trait]
impl CommandHandler for PingCommand {
    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        _: &GuildConfig,
        message: &Message,
        _: &[&str]
    ) {
        println!("ping command acked");
        // For a text message with an embed:
        let _ = client.send_message(
            message.channel_id,
            MessageContent::TextAndEmbed("pong".to_string(), DiscordEmbed {
                title: Some("title".to_string()),
                ..Default::default()
            })
        );
    }
}
