use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use songbird::tracks::PlayMode;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
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

        let (key, color) = if let Some(_) = client.voice_music_manager.songbird.get(guild_id) {
            if !client.is_user_in_same_channel_as_bot(guild_id, msg.author.id).await? {
                ("music-not-same-channel", ColorResolvables::Red)
            } else {
                // Scope to limit the lock guard
                let track_queue = {
                    let store = client.voice_music_manager.trackqueues.read().unwrap();
                    store.get(&guild_id).cloned()
                };

                if let Some(trackqueue) = track_queue {
                    if let Some(handle) = trackqueue.current() {
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
                        if !paused {
                            ("command-pause-paused", ColorResolvables::Yellow)
                        } else {
                            ("command-pause-unpaused", ColorResolvables::Green)
                        }
                    } else {
                        ("music-not-playing", ColorResolvables::Red)
                    }
                } else {
                    ("music-not-playing", ColorResolvables::Red)
                }
            }
        } else {
            ("music-not-playing", ColorResolvables::Red)
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
