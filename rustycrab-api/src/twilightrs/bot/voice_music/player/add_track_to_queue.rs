use std::sync::Arc;

use rustycrab_model::error::BoxedError;
use songbird::{ Event, TrackEvent, input::{ YoutubeDl, Compose } };
use twilight_model::{ user::User, id::{ marker::{ ChannelMarker, GuildMarker }, Id } };

use crate::twilightrs::{
    discord_client::DiscordClient,
    bot::voice_music::player::track_events::MusicEventHandler,
};

pub async fn add_track_to_queue(
    client: DiscordClient,
    channel_id: Id<ChannelMarker>,
    guild_id: Id<GuildMarker>,
    requested_by: &User,
    url: String
) -> Result<(), BoxedError> {
    // Create a new source from the URL
    let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());

    // Fetch metadata for the new track
    let metadata = source.aux_metadata().await?;
    if let Some(call_lock) = client.voice_music_manager.songbird.get(guild_id) {
        // Add the source to the track queue
        let mut call = call_lock.lock().await;

        // We need to ensure that this guild has a TrackQueue created for it.
        let track_queue = {
            let mut queues = client.voice_music_manager.trackqueues.write().unwrap();
            queues.entry(guild_id).or_default().clone()
        };
        let handle = track_queue.add_source(source.into(), &mut *call).await;

        let mut player_channels = client.voice_music_manager.music_player_channel_ids
            .write()
            .unwrap();

        let channel_id = if let Some(channel_id) = player_channels.get(&guild_id) {
            channel_id.clone()
        } else {
            player_channels.entry(guild_id).or_insert(channel_id);
            channel_id
        };

        println!("music player channels {player_channels:?}");

        let event_handler = MusicEventHandler {
            client: Arc::clone(&client),
            channel_id,
            guild_id,
            url: url.to_string(),
            metadata: metadata.clone(),
            requested_by: requested_by.clone(),
            // message_id: None,
        };

        handle
            .add_event(Event::Track(TrackEvent::Play), event_handler)
            .expect("Failed to add event handler");

        let event_handler = MusicEventHandler {
            client: Arc::clone(&client),
            channel_id,
            guild_id,
            url: url.to_string(),
            metadata: metadata.clone(),
            requested_by: requested_by.clone(),
            // message_id: None,
        };

        handle
            .add_event(Event::Track(TrackEvent::End), event_handler)
            .expect("Failed to add event handler");

        let event_handler = MusicEventHandler {
            client: Arc::clone(&client),
            channel_id,
            guild_id,
            url: url.to_string(),
            metadata: metadata.clone(),
            requested_by: requested_by.clone(),
            // message_id: None,
        };
        handle
            .add_event(Event::Track(TrackEvent::Pause), event_handler)
            .expect("Failed to add event handler");
    }

    Ok(())
}
