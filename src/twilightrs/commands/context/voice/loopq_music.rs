use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
        bot::voice_music::voice_manager::PlayerLoopState,
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
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

        let (key, color) = if let Some(_) = client.voice_music_manager.songbird.get(guild_id) {
            if !client.is_user_in_same_channel_as_bot(guild_id, msg.author.id).await? {
                ("music-not-same-channel", ColorResolvables::Red)
            } else {
                client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::LoopQueue);
                ("command-loop-queue", ColorResolvables::Green)
            }
        } else {
            ("music-no-voice", ColorResolvables::Red)
        };

        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    description: Some(client.get_locale_string(&config.locale, key, None)),
                    color: Some(color.as_u32()),
                    footer_text: Some(
                        client.get_locale_string(
                            &config.locale,
                            "requested-user",
                            Some(
                                &FluentArgs::from_iter(vec![("username", msg.author.name.clone())])
                            )
                        )
                    ),
                    footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }
}
