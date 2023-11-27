use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{
        context_command::{ ContextCommand, GuildConfigModel },
        ParsedArg,
        ArgSpec,
        ArgType,
    },
    discord_client::{ DiscordClient, MessageContent },
};

pub struct JoinCommand {}

#[async_trait]
impl ContextCommand for JoinCommand {
    fn name(&self) -> &'static str {
        "join"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("voice channel", ArgType::Channel, true)]
    }

    async fn run(
        &self,
        client: DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // Ensure the user is in a voice channel
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => {
                return Ok(());
            } // Command not used in a guild
        };

        let channel_id = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => state.channel_id(),
            None => {
                // Notify user they need to be in a voice channel
                client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::Text(
                        "You need to be in a voice channel to use the command".to_string()
                    )
                ).await?;
                return Ok(());
            }
        };

        let join_result = client.voice_manager.songbird.join(guild_id, channel_id).await;

        let content = match join_result {
            Ok(_call_lock) => {
                // Successfully joined the channel
                format!("Joined <#{}>!", channel_id)
            }
            Err(e) => {
                // Failed to join the channel
                format!("Failed to join <#{}>! Why: {:?}", channel_id, e)
            }
        };

        // Notify user about the result
        client.reply_message(msg.channel_id, msg.id, MessageContent::Text(content)).await?;

        Ok(())
    }
}

impl JoinCommand {}
