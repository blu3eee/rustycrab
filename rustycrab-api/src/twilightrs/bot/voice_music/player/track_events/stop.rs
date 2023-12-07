use rustycrab_model::{ error::BoxedError, music::PlayerLoopState };

use crate::twilightrs::bot::voice_music::player::add_next_track::add_next_track;

use super::MusicEventHandler;

pub async fn handle_event_track_end(
    music_event_handler: &MusicEventHandler,
    _locale: &str
) -> Result<(), BoxedError> {
    // Logic to execute when a track ends

    // if the Player's looping state is queue, add the url of just finished track to the end of the waiting queue
    match
        music_event_handler.client.voice_music_manager.get_loop_state(music_event_handler.guild_id)
    {
        PlayerLoopState::LoopQueue => {
            // Re-add the current song to the end of the queue
            music_event_handler.client.voice_music_manager.extend_waiting_queue(
                music_event_handler.guild_id,
                &vec![music_event_handler.url.clone()]
            );
        }
        _ => {
            // ignore
        }
    }
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
    // When a track ends, check for the next URL in waiting_track_urls
    music_event_handler.client.voice_music_manager.set_current_song(
        music_event_handler.guild_id,
        None
    );

    // Add next track to the queue
    if let Err(e) = add_next_track(&music_event_handler).await {
        eprintln!(
            "[Guild: {}] Error adding next track to queue: {:?}",
            music_event_handler.guild_id,
            e
        );
    }
    Ok(())
}
