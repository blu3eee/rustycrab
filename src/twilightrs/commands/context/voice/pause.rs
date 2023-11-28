use std::error::Error;

use async_trait::async_trait;
use songbird::tracks::PlayMode;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::DiscordClient,
        utils::reply_command,
    },
    utilities::utils::ColorResolvables,
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
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let handle = client.fetch_trackhandle(guild_id, Some(&config.locale)).await?;

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

        let (key, color) = {
            if !paused {
                ("command-pause-paused", ColorResolvables::Yellow)
            } else {
                ("command-pause-unpaused", ColorResolvables::Green)
            }
        };

        reply_command(&client, config, msg, key, None, color).await?;

        Ok(())
    }
}
