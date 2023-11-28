use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::DiscordClient,
        bot::voice_music::voice_manager::PlayerLoopState,
        utils::send_response_message,
    },
    utilities::utils::ColorResolvables,
};
pub struct LoopQueueMusicCommand {}

#[async_trait]
impl ContextCommand for LoopQueueMusicCommand {
    fn name(&self) -> &'static str {
        "loopq"
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

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let (key, color) = {
            client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::LoopQueue);
            ("command-loop-queue", ColorResolvables::Green)
        };

        send_response_message(&client, config, msg, key, color).await?;

        Ok(())
    }
}
