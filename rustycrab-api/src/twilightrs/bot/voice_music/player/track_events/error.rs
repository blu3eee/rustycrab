use rustycrab_model::{ error::BoxedError, color::ColorResolvables };

use crate::twilightrs::{
    bot::voice_music::player::add_next_track::add_next_track,
    messages::DiscordEmbed,
};

use super::MusicEventHandler;

pub async fn handle_event_track_error(
    music_event_handler: &MusicEventHandler,
    locale: &str
) -> Result<(), BoxedError> {
    // Logic to execute when an error happened loading the track

    if
        let Some(message_id) =
            music_event_handler.client.voice_music_manager.get_music_player_message(
                music_event_handler.guild_id
            )
    {
        let _ = music_event_handler.client.http.delete_message(
            music_event_handler.channel_id,
            message_id
        ).await;
    }

    let _ = music_event_handler.client.send_message(
        music_event_handler.channel_id,
        crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
            vec![DiscordEmbed {
                description: Some(
                    format!(
                        "[**{}**]{}: {}",
                        music_event_handler.metadata.title
                            .as_ref()
                            .unwrap_or(&"<UNKNOWN>".to_string())
                            .to_string(),
                        music_event_handler.metadata.source_url.clone().map_or_else(
                            || String::new(),
                            |url| format!("({})", url)
                        ),
                        music_event_handler.client.get_locale_string(
                            &locale,
                            "music-error-track",
                            None
                        )
                    )
                ),
                color: Some(ColorResolvables::Red.as_u32()),
                ..Default::default()
            }]
        )
    ).await;
    // When a track ends, check for the next URL in waiting_track_urls
    music_event_handler.client.voice_music_manager.set_current_song(
        music_event_handler.guild_id,
        None
    );

    // Add next track to the queue
    if let Err(e) = add_next_track(music_event_handler).await {
        eprintln!(
            "[Guild: {}] Error adding next track to queue: {:?}",
            music_event_handler.guild_id,
            e
        );
    }
    Ok(())
}
