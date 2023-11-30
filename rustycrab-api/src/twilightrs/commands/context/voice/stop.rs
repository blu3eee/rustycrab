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
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        let call_lock = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let handle = client.fetch_trackhandle(guild_id, Some(&config.locale)).await?;

        let mut call = call_lock.lock().await;

        client.voice_music_manager.clear_waiting_queue(guild_id);
        call.stop();
        let _ = handle.stop();

        client.http.create_message(msg.channel_id).content("Stopped playing music")?.await?;

        Ok(())
    }
}
