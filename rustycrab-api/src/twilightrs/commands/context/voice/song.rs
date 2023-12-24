use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use twilight_model::{ gateway::payload::incoming::MessageCreate, channel::message::Embed };

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::DiscordClient,
        messages::DiscordEmbed,
        bot::voice_music::player::track_info::track_info_fields,
    },
    cdn_avatar,
};

pub struct CurrentSongCommand {}

#[async_trait]
impl ContextCommand for CurrentSongCommand {
    fn name(&self) -> &'static str {
        "song"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["nowplaying", "track", "playing"]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        let _ = client.voice_music_manager.fetch_call_lock(guild_id).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id).await?;

        let _ = client.voice_music_manager.fetch_trackhandle(guild_id).await?;

        let current_track = client.voice_music_manager.get_current_song(guild_id);

        if let Some((metadata, requested_by)) = current_track {
            client.http.create_message(msg.channel_id).embeds(
                &vec![
                    Embed::from(DiscordEmbed {
                        author_name: Some(
                            client.get_locale_string(&config.locale, "music-nowplaying", None)
                        ),
                        footer_text: Some(
                            client.get_locale_string(
                                &config.locale,
                                "requested-user",
                                Some(
                                    &FluentArgs::from_iter(
                                        vec![("username", msg.author.name.clone())]
                                    )
                                )
                            )
                        ),
                        author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
                        thumbnail: metadata.thumbnail.clone(),
                        fields: Some(track_info_fields(&client, &config.locale, &metadata, None)),
                        footer_icon_url: requested_by.avatar.map(|hash|
                            cdn_avatar!(requested_by.id, hash)
                        ),
                        color: Some(ColorResolvables::Green.as_u32()),
                        ..Default::default()
                    })
                ]
            )?.await?;
        }

        Ok(())
    }
}
