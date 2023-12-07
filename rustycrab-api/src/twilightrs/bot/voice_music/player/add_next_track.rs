use std::sync::Arc;

use rustycrab_model::{ error::BoxedError, color::ColorResolvables };
use twilight_model::channel::message::Embed;

use crate::twilightrs::messages::DiscordEmbed;

use super::{ add_track_to_queue::add_track_to_queue, track_events::MusicEventHandler };

pub async fn add_next_track(music_event_handler: &MusicEventHandler) -> Result<(), BoxedError> {
    // Add next track to the queue
    loop {
        if
            let Some(next_url) = music_event_handler.client.voice_music_manager.pop_next_track_url(
                music_event_handler.guild_id
            )
        {
            if
                let Err(e) = add_track_to_queue(
                    Arc::clone(&music_event_handler.client),
                    music_event_handler.channel_id,
                    music_event_handler.guild_id,
                    &music_event_handler.requested_by,
                    next_url
                ).await
            {
                eprintln!("Error adding next track to queue: {:?}", e);
            } else {
                break;
            }
        } else {
            if
                let Ok(message) = music_event_handler.client.http
                    .create_message(music_event_handler.channel_id)
                    .embeds(
                        &vec![
                            Embed::from(DiscordEmbed {
                                description: Some(format!("No more track to play..")),
                                color: Some(ColorResolvables::Red.as_u32()),
                                ..Default::default()
                            })
                        ]
                    )
            {
                let _ = message.await;
                let mut player_channels =
                    music_event_handler.client.voice_music_manager.music_player_channel_ids
                        .write()
                        .unwrap();
                player_channels.remove(&music_event_handler.guild_id);
                break;
            }
        }
    }
    Ok(())
}
