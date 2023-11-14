use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::{ DiscordClient, MessageContent },
        embeds::DiscordEmbed,
    },
    cdn_avatar,
};

pub struct SnipeCommand;

#[async_trait]
impl ContextCommand for SnipeCommand {
    fn name(&self) -> &'static str {
        "snipe"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new(ArgType::User, true), ArgSpec::new(ArgType::Number, true)]
    }

    async fn run(
        &self,
        client: &DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let position = if let Some(ParsedArg::Number(pos)) = command_args.get(0) {
            *pos as usize
        } else {
            0 // Default to the most recent deleted message
        };

        // Extracting the deleted messages within a smaller scope to ensure the lock is released before await
        let sniped_message = {
            let deleted_messages = client.deleted_messages.read().unwrap();
            deleted_messages.get(&msg.channel_id).and_then(|messages| {
                if position < messages.len() {
                    Some(messages[messages.len() - 1 - position].clone())
                } else {
                    None
                }
            })
        };

        if let Some(message) = sniped_message {
            if let Some(message_author) = client.cache.user(message.author()) {
                let message_user = message_author.value();
                client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::DiscordEmbeds(
                        vec![DiscordEmbed {
                            description: if message.content().len() > 0 {
                                Some(message.content().to_string())
                            } else {
                                None
                            },
                            author_name: Some(message_user.name.to_string()),
                            author_icon_url: message_user.avatar.map(|avatar_hash|
                                cdn_avatar!(message_user.id, avatar_hash)
                            ),
                            footer_text: Some(format!("Sniped by @{}", msg.author.name)),
                            timestamp: Some(true),
                            ..Default::default()
                        }]
                    )
                ).await?;
            }
        } else {
            client.send_message(
                msg.channel_id,
                MessageContent::Text(
                    "No deleted message found at the specified position.".to_string()
                )
            ).await?;
        }

        Ok(())
    }
}
