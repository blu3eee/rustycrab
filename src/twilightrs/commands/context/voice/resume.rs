use std::error::Error;

use async_trait::async_trait;
use songbird::tracks::PlayMode;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
};
pub struct ResumeMusicCommand {}

#[async_trait]
impl ContextCommand for ResumeMusicCommand {
    fn name(&self) -> &'static str {
        "resume"
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
        let trackqueue = {
            let store = client.trackqueues.read().unwrap();
            store.get(&guild_id).cloned()
        };

        if let Some(tracks_queue) = trackqueue {
            if let Some(handle) = tracks_queue.current() {
                let info = handle.get_info().await?;

                if info.playing == PlayMode::Pause {
                    let _success = handle.play();
                    let _ = client.http
                        .create_message(msg.channel_id)
                        .content("Resumed the track")?.await;
                    return Ok(());
                }
            }
        }
        client.http.create_message(msg.channel_id).content("No track is currently paused")?.await?;
        Ok(())
    }
}
