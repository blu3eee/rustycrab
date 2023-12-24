use std::error::Error;

use async_trait::async_trait;
use rustycrab_model::{ color::ColorResolvables, music::PlayerLoopState };
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
pub struct LoopMusicCommand {}

#[async_trait]
impl ContextCommand for LoopMusicCommand {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("type: current/one/all/queue", ArgType::Arg, true)]
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

        let track_handle = client.voice_music_manager.fetch_trackhandle(guild_id).await?;

        let loop_type = if let Some(ParsedArg::Arg(state)) = command_args.first() {
            state
        } else {
            "queue"
        };

        let (key, color) = match loop_type {
            "current" | "song" | "track" | "one" | "1" => {
                if let Ok(_) = track_handle.enable_loop() {
                    client.voice_music_manager.set_loop_state(
                        guild_id,
                        PlayerLoopState::LoopCurrentTrack
                    );
                    ("command-loop-track", ColorResolvables::Green)
                } else {
                    ("command-loop-track-failed", ColorResolvables::Red)
                }
            }
            "queue" | "all" => {
                client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::LoopQueue);

                ("command-loop-queue", ColorResolvables::Green)
            }
            _ => { ("command-loop-invalid", ColorResolvables::Red) }
        };

        reply_command(&client, config, msg, key, None, color).await?;

        Ok(())
    }
}
