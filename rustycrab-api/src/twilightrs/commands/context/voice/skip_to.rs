use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{
        context_command::{ ContextCommand, GuildConfigModel },
        ParsedArg,
        ArgSpec,
        ArgType,
    },
    discord_client::DiscordClient,
    utils::reply_command,
};
pub struct SkipToTrackCommand {}

#[async_trait]
impl ContextCommand for SkipToTrackCommand {
    fn name(&self) -> &'static str {
        "skipto"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("position", ArgType::Number, false)]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        let _ = client.voice_music_manager.fetch_call_lock(guild_id).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id).await?;

        let handle = client.voice_music_manager.fetch_trackhandle(guild_id).await?;

        let mut args = FluentArgs::new();

        let (key, color) = match command_args.first() {
            Some(ParsedArg::Number(position)) => {
                // Skips to the specified track position (indexed-1)
                if client.voice_music_manager.skip_to_position(guild_id, *position as usize) {
                    args.set("position", position);
                    // Skip the current track
                    let _ = handle.stop();

                    ("command-skipto-success", ColorResolvables::Red)
                } else {
                    args.set("count", client.voice_music_manager.get_waiting_queue(guild_id).len());
                    ("command-skipto-invalid", ColorResolvables::Red)
                }
            }
            _ => { ("command-skipto-nopos", ColorResolvables::Red) }
        };
        reply_command(&client, config, msg, key, Some(args), color).await?;
        Ok(())
    }
}
