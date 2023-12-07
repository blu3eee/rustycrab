use fluent_bundle::FluentArgs;
use rustycrab_model::{ error::BoxedError, color::ColorResolvables };
use twilight_model::channel::message::Embed;

use crate::{
    twilightrs::{ bot::voice_music::player::track_info::track_info_fields, messages::DiscordEmbed },
    cdn_avatar,
};

use super::MusicEventHandler;

pub async fn handle_event_track_play(
    music_event_handler: &MusicEventHandler,
    locale: &str
) -> Result<(), BoxedError> {
    // Logic to execute when a track starts playing.
    music_event_handler.client.voice_music_manager.set_current_song(
        music_event_handler.guild_id,
        Some((music_event_handler.metadata.clone(), &music_event_handler.requested_by))
    );
    let guild = if
        let Ok(guild) = music_event_handler.client.http.guild(music_event_handler.guild_id).await
    {
        if let Ok(guild) = guild.model().await {
            guild.name
        } else {
            music_event_handler.guild_id.to_string()
        }
    } else {
        music_event_handler.guild_id.to_string()
    };

    println!(
        "[Guild: {}] Track started playing: {:?}",
        guild,
        music_event_handler.metadata.title.clone().unwrap_or(format!("unknown"))
    );

    let embed = DiscordEmbed {
        author_name: Some(
            music_event_handler.client.get_locale_string(&locale, "music-nowplaying", None)
        ),
        author_icon_url: Some(music_event_handler.client.voice_music_manager.spinning_disk.clone()),
        thumbnail: if let Some(url) = &music_event_handler.metadata.thumbnail {
            Some(url.to_string())
        } else {
            None
        },
        fields: Some(
            track_info_fields(
                &music_event_handler.client,
                &locale,
                &music_event_handler.metadata,
                None
            )
        ),
        footer_text: Some(
            music_event_handler.client.get_locale_string(
                &locale,
                "requested-user",
                Some(
                    &FluentArgs::from_iter(
                        vec![("username", music_event_handler.requested_by.name.clone())]
                    )
                )
            )
        ),
        footer_icon_url: music_event_handler.requested_by.avatar.map(|hash|
            cdn_avatar!(music_event_handler.requested_by.id, hash)
        ),
        color: Some(ColorResolvables::Green.as_u32()),
        ..Default::default()
    };

    if
        let Ok(message) = music_event_handler.client.http
            .create_message(music_event_handler.channel_id)
            .embeds(&vec![Embed::from(embed)])
    {
        if let Ok(message) = message.await {
            if let Ok(message) = message.model().await {
                // music_event_handler.message_id = Some(message.id);
                music_event_handler.client.voice_music_manager.set_music_player_message(
                    music_event_handler.guild_id,
                    message.id
                );
            }
        }
    }
    Ok(())
}
