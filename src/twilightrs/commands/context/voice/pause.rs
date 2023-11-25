use std::error::Error;

use async_trait::async_trait;
use songbird::tracks::PlayMode;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
};
pub struct PauseMusicCommand {}

#[async_trait]
impl ContextCommand for PauseMusicCommand {
    fn name(&self) -> &'static str {
        "pause"
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
        // Scope to limit the lock guard
        let track_handle = {
            let store = client.trackdata.read().unwrap();
            store.get(&guild_id).cloned()
        };

        if let Some(handle) = track_handle {
            let info = handle.get_info().await?;

            let paused = match info.playing {
                PlayMode::Play => {
                    let _success = handle.pause();
                    false
                }
                _ => {
                    let _success = handle.play();
                    true
                }
            };
            let action = if paused { "Unpaused" } else { "Paused" };
            let _ = client.http
                .create_message(msg.channel_id)
                .content(&format!("{} the track", action))?.await;
        } else {
            client.http
                .create_message(msg.channel_id)
                .content("No music track is currently playing")?.await?;
        }

        Ok(())
    }
}
