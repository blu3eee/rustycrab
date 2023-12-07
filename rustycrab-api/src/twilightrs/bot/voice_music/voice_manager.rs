use std::{ sync::{ Arc, RwLock }, collections::HashMap };

use rustycrab_model::{ music::PlayerLoopState, error::BoxedError };
use songbird::{ Songbird, tracks::{ TrackQueue, PlayMode, TrackHandle }, input::AuxMetadata };
use twilight_model::{
    id::{ marker::{ GuildMarker, ChannelMarker, MessageMarker }, Id },
    user::User,
};

/// VoiceManager is a central manager for all voice-related functionalities in a Discord bot.
/// It encapsulates various aspects of voice interaction, such as song queue management,
/// event handling, and current song tracking.
pub struct VoiceManager {
    /// An Arc-wrapped instance of Songbird for managing voice state and audio processing.
    pub songbird: Arc<Songbird>,
    /// A thread-safe map from guild IDs to their respective TrackQueue instances.
    pub trackqueues: RwLock<HashMap<Id<GuildMarker>, TrackQueue>>,
    /// A thread-safe map from guild IDs to queues of upcoming track URLs (Vec<String>).
    pub waiting_track_urls: RwLock<HashMap<Id<GuildMarker>, Vec<String>>>,
    /// A thread-safe map for managing music-related event handlers in each guild.
    pub music_event_handlers: RwLock<HashMap<Id<GuildMarker>, TrackQueue>>,
    /// A thread-safe map tracking the currently playing song's metadata and the user who requested it in each guild.
    pub current_song: RwLock<HashMap<Id<GuildMarker>, (AuxMetadata, User)>>,
    pub loop_state: RwLock<HashMap<Id<GuildMarker>, PlayerLoopState>>,
    pub music_player_message_ids: RwLock<HashMap<Id<GuildMarker>, Id<MessageMarker>>>,
    pub music_player_channel_ids: RwLock<HashMap<Id<GuildMarker>, Id<ChannelMarker>>>,
    /// A String URL for a spinning disk icon used in UI elements.
    pub spinning_disk: String,
}

impl VoiceManager {
    /// Constructor for VoiceManager. Initializes the fields with default values and the provided Songbird instance.
    pub fn new(songbird: Arc<Songbird>) -> Self {
        Self {
            songbird,
            trackqueues: Default::default(),
            waiting_track_urls: Default::default(),
            music_event_handlers: Default::default(),
            current_song: Default::default(),
            loop_state: Default::default(),
            music_player_message_ids: Default::default(),
            music_player_channel_ids: Default::default(),
            spinning_disk: "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string(),
        }
    }

    /// Extends the waiting track queue for a given guild with additional URLs.
    pub fn extend_waiting_queue(&self, guild_id: Id<GuildMarker>, urls: &Vec<String>) {
        let mut waiting_urls = self.waiting_track_urls.write().unwrap();
        waiting_urls.entry(guild_id).or_default().extend(urls.clone());
    }

    /// Clears the waiting track queue for the specified guild.
    pub fn clear_waiting_queue(&self, guild_id: Id<GuildMarker>) {
        let mut waiting_urls = self.waiting_track_urls.write().unwrap();
        waiting_urls.remove(&guild_id);

        let trackqueue = self.get_play_queue(guild_id);
        trackqueue.stop();
    }

    /// Removes and returns the next track URL from the waiting queue for the specified guild.
    pub fn pop_next_track_url(&self, guild_id: Id<GuildMarker>) -> Option<String> {
        let mut waiting_urls = self.waiting_track_urls.write().unwrap();
        waiting_urls.get_mut(&guild_id).and_then(|urls| {
            if let Some(first) = urls.first() {
                let first = first.to_string();
                urls.remove(0);
                Some(first)
            } else {
                None
            }
        })
    }

    /// Retrieves the play queue (TrackQueue) for a specified guild.
    pub fn get_play_queue(&self, guild_id: Id<GuildMarker>) -> TrackQueue {
        let mut queues = self.trackqueues.write().unwrap();
        queues.entry(guild_id).or_default().clone()
    }

    /// Retrieves the list of URLs in the waiting queue for a specified guild.
    pub fn get_waiting_queue(&self, guild_id: Id<GuildMarker>) -> Vec<String> {
        let mut waiting_urls = self.waiting_track_urls.write().unwrap();
        waiting_urls.entry(guild_id).or_default().clone()
    }

    /// Sets the metadata of the current song for a specified guild, along with the user who requested it.
    pub fn set_current_song(
        &self,
        guild_id: Id<GuildMarker>,
        current_info: Option<(AuxMetadata, &User)>
    ) {
        let mut current_song = self.current_song.write().unwrap();
        if let Some((metadata, requested_by)) = current_info {
            current_song.insert(guild_id, (metadata, requested_by.clone()));
        } else {
            current_song.remove(&guild_id);
        }
    }

    /// Retrieves the metadata of the current song and the user who requested it for a specified guild.
    pub fn get_current_song(&self, guild_id: Id<GuildMarker>) -> Option<(AuxMetadata, User)> {
        let current_song = self.current_song.read().unwrap();
        current_song.get(&guild_id).cloned()
    }

    /// Skips to a specific position in the waiting queue.
    ///
    /// ### Parameters
    /// - `guild_id`: The ID of the guild.
    /// - `position`: The position in the queue to skip to (1-indexed).
    ///
    /// ### Returns
    /// - `bool`: Returns `true` if the skip was successful, `false` otherwise.
    pub fn skip_to_position(&self, guild_id: Id<GuildMarker>, position: usize) -> bool {
        let mut waiting_urls = self.waiting_track_urls.write().unwrap();
        if let Some(queue) = waiting_urls.get_mut(&guild_id) {
            if position == 0 || position > queue.len() {
                return false;
            }
            queue.drain(0..position - 1);
            true
        } else {
            false
        }
    }

    pub fn set_loop_state(&self, guild_id: Id<GuildMarker>, state: PlayerLoopState) {
        let mut loop_state = self.loop_state.write().unwrap();
        loop_state.insert(guild_id, state);
    }

    pub fn get_loop_state(&self, guild_id: Id<GuildMarker>) -> PlayerLoopState {
        let loop_state = self.loop_state.read().unwrap();
        loop_state.get(&guild_id).cloned().unwrap_or(PlayerLoopState::NoLoop)
    }

    pub fn set_music_player_message(
        &self,
        guild_id: Id<GuildMarker>,
        message_id: Id<MessageMarker>
    ) {
        let mut player = self.music_player_message_ids.write().unwrap();
        player.insert(guild_id, message_id);
    }

    pub fn get_music_player_message(&self, guild_id: Id<GuildMarker>) -> Option<Id<MessageMarker>> {
        let player = self.music_player_message_ids.read().unwrap();
        player.get(&guild_id).cloned()
    }

    /// pause a player if it is playing, return a `bool` determine if the player is paused from playing state
    pub async fn pause_player(&self, guild_id: Id<GuildMarker>) -> Result<bool, BoxedError> {
        if let Ok(handle) = self.fetch_trackhandle(guild_id).await {
            let info = handle.get_info().await?;
            match info.playing {
                PlayMode::Play => {
                    return Ok(handle.pause().is_ok());
                }
                _ => {
                    return Ok(false);
                }
            };
        }
        Ok(false)
    }

    pub async fn fetch_trackhandle(
        &self,
        guild_id: Id<GuildMarker>
    ) -> Result<TrackHandle, BoxedError> {
        let track_queue = {
            let store = self.trackqueues.read().unwrap();
            store.get(&guild_id).cloned()
        };
        if let Some(trackqueue) = track_queue {
            if let Some(handle) = trackqueue.current() {
                return Ok(handle);
            }
        }

        return Err("music-not-playing".into());
    }

    /// resume player if the player is paused, return a `bool` determine if the player is resumed from pause state
    pub async fn resume_player(&self, guild_id: Id<GuildMarker>) -> Result<bool, BoxedError> {
        if let Ok(handle) = self.fetch_trackhandle(guild_id).await {
            let info = handle.get_info().await?;
            match info.playing {
                PlayMode::Pause => {
                    return Ok(handle.play().is_ok());
                }
                _ => {
                    return Ok(false);
                }
            };
        }

        return Ok(false);
    }

    pub fn get_player_ids(
        &self,
        guild_id: Id<GuildMarker>
    ) -> (Option<Id<ChannelMarker>>, Option<Id<MessageMarker>>) {
        let channel_id = {
            let player_channels = self.music_player_channel_ids.read().unwrap();
            player_channels.get(&guild_id).cloned()
        };

        let message_id = {
            let player_channels = self.music_player_message_ids.read().unwrap();
            player_channels.get(&guild_id).cloned()
        };

        (channel_id, message_id)
    }
}
