use std::error::Error;

use async_trait::async_trait;
use rustycrab_model::music::PlayerLoopState;
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
        _config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;
        let _ = client.voice_music_manager.fetch_call_lock(guild_id).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id).await?;

        let handle = client.voice_music_manager.fetch_trackhandle(guild_id).await?;

        client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::NoLoop);
        client.voice_music_manager.clear_waiting_queue(guild_id);

        let _ = handle.stop();

        client.http.create_message(msg.channel_id).content("Stopped playing music")?.await?;

        Ok(())
    }
}
