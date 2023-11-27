use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{
            context_command::{ ContextCommand, GuildConfigModel },
            ParsedArg,
            ArgSpec,
            ArgType,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
        bot::voice_manager::PlayerLoopState,
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
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
        _: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
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

        let loop_type = if let Some(ParsedArg::Arg(state)) = command_args.first() {
            state
        } else {
            "queue"
        };

        let (response, color) = match loop_type {
            "current" | "song" | "track" | "one" | "1" => {
                if let Some(track_handle) = client.voice_manager.get_play_queue(guild_id).current() {
                    track_handle.enable_loop()?;
                    client.voice_manager.set_loop_state(
                        guild_id,
                        PlayerLoopState::LoopCurrentTrack
                    );
                    ("Looping current track", ColorResolvables::Green)
                } else {
                    ("Failed to loop current track", ColorResolvables::Red)
                }
            }
            "queue" | "all" => {
                client.voice_manager.set_loop_state(guild_id, PlayerLoopState::LoopQueue);

                ("Looping entire queue", ColorResolvables::Green)
            }
            _ => { ("Invalid loop type", ColorResolvables::Red) }
        };

        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    description: Some(response.to_string()),
                    color: Some(color.as_u32()),
                    footer_text: Some(format!("Requested by @{}", msg.author.name)),
                    footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }
}
