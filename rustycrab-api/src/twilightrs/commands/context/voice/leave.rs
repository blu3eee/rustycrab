use std::error::Error;

use async_trait::async_trait;
use rustycrab_model::{ color::ColorResolvables, music::PlayerLoopState };
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
    utils::reply_command,
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
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;

        // let _ = client.fetch_trackhandle(guild_id, Some(&config.locale)).await?;

        client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::NoLoop);
        client.voice_music_manager.clear_waiting_queue(guild_id);

        let (key, color) = {
            if client.voice_music_manager.songbird.remove(guild_id).await.is_ok() {
                // let mut call = call_lock.lock().await;
                ("command-leave-left", ColorResolvables::Green)
            } else {
                ("command-leave-failed", ColorResolvables::Red)
            }
        };

        reply_command(&client, config, msg, key, None, color).await?;

        Ok(())
    }
}
