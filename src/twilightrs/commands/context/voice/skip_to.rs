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
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
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

        let position = match command_args.first() {
            Some(ParsedArg::Number(pos)) => *pos as usize,
            _ => {
                client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::Text("Please provide a valid position to skip to".to_string())
                ).await?;
                return Ok(());
            }
        };

        // Skips to the specified track position (indexed-1)
        if client.voice_music_manager.skip_to_position(guild_id, position) {
            // Skip the current track
            if let Some(handle) = client.voice_music_manager.get_play_queue(guild_id).current() {
                let _ = handle.stop();
            }

            client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some(format!("Skipped to track at position {}", position)),
                        footer_text: Some(format!("Skipped by @{}", msg.author.name)),
                        footer_icon_url: msg.author.avatar.map(|hash|
                            cdn_avatar!(msg.author.id, hash)
                        ),
                        color: Some(ColorResolvables::Green.as_u32()),
                        ..Default::default()
                    }]
                )
            ).await?;
        } else {
            client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::Text("Invalid position provided".to_string())
            ).await?;
        }

        Ok(())
    }
}
