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
        discord_client::DiscordClient,
        utils::send_response_message,
    },
    utilities::utils::ColorResolvables,
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
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let handle = client.fetch_trackhandle(guild_id, Some(&config.locale)).await?;

        let (key, color) = match command_args.first() {
            Some(ParsedArg::Number(position)) => {
                // Skips to the specified track position (indexed-1)
                if client.voice_music_manager.skip_to_position(guild_id, *position as usize) {
                    // Skip the current track
                    let _ = handle.stop();

                    ("command-skipto-success", ColorResolvables::Red)
                } else {
                    ("command-skipto-invalid", ColorResolvables::Red)
                }
            }
            _ => { ("command-skipto-nopos", ColorResolvables::Red) }
        };
        send_response_message(&client, config, msg, key, color).await?;
        Ok(())
    }
}
