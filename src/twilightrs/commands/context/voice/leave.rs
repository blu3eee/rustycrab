use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
};
pub struct LeaveChannelCommand {}

#[async_trait]
impl ContextCommand for LeaveChannelCommand {
    fn name(&self) -> &'static str {
        "leave"
    }

    async fn run(
        &self,
        client: DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("Command not used in a guild")?;
        if !client.is_user_in_same_channel_as_bot(guild_id, msg.author.id).await? {
            client.http
                .create_message(msg.channel_id)
                .content(
                    "You need to be in the same voice channel as the bot to use this command"
                )?.await?;
            return Ok(());
        }

        if client.songbird.leave(guild_id).await.is_ok() {
            client.http.create_message(msg.channel_id).content("Left the voice channel")?.await?;
        } else {
            client.http
                .create_message(msg.channel_id)
                .content("Failed to leave the voice channel")?.await?;
        }

        Ok(())
    }
}
