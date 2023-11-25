use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
};

pub struct StopMusicCommand {}

#[async_trait]
impl ContextCommand for StopMusicCommand {
    fn name(&self) -> &'static str {
        "stop"
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

        if let Some(call_lock) = client.songbird.get(guild_id) {
            let mut call = call_lock.lock().await;
            call.stop();

            client.http.create_message(msg.channel_id).content("Stopped playing music")?.await?;
        } else {
            client.http
                .create_message(msg.channel_id)
                .content("No music is currently playing")?.await?;
        }

        Ok(())
    }
}
