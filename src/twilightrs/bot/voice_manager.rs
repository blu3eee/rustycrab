use std::{ sync::{ Arc, RwLock }, error::Error, collections::HashMap };

use async_trait::async_trait;
use songbird::{
    Songbird,
    tracks::TrackQueue,
    input::{ YoutubeDl, Compose, AuxMetadata },
    TrackEvent,
    Event,
    EventHandler as SongbirdEventHandler,
    EventContext,
    tracks::PlayMode,
};
use twilight_model::{
    id::{ marker::{ GuildMarker, ChannelMarker }, Id },
    user::User,
    channel::message::Embed,
};
use twilight_http::Client as HttpClient;

use crate::{
    twilightrs::messages::{ DiscordEmbed, DiscordEmbedField },
    utilities::{ format_duration, utils::ColorResolvables },
    cdn_avatar,
};

#[derive(Clone, Debug)]
pub enum PlayerLoopState {
    NoLoop,
    LoopCurrentTrack,
    LoopQueue,
}

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
}

struct MusicEventHandler {
    pub http: Arc<HttpClient>,
    pub music_manager: Arc<VoiceManager>,
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    pub url: String,
    pub metadata: AuxMetadata,
    pub requested_by: User,
}

#[async_trait]
impl SongbirdEventHandler for MusicEventHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_states) => {
                for (track_state, _) in *track_states {
                    // Logic to execute when a track starts playing.
                    // You can check specific conditions here, for example:
                    if track_state.playing == PlayMode::Play {
                        self.music_manager.set_current_song(
                            self.guild_id,
                            Some((self.metadata.clone(), &self.requested_by))
                        );
                        let guild = if let Ok(guild) = self.http.guild(self.guild_id).await {
                            if let Ok(guild) = guild.model().await {
                                guild.name
                            } else {
                                self.guild_id.to_string()
                            }
                        } else {
                            self.guild_id.to_string()
                        };

                        println!(
                            "[Guild: {}] Track started playing: {:?}",
                            guild,
                            self.metadata.title.clone().unwrap_or(format!("unknown"))
                        );

                        if
                            let Ok(message) = self.http.create_message(self.channel_id).embeds(
                                &vec![
                                    Embed::from(DiscordEmbed {
                                        author_name: Some("Now playing".to_string()),
                                        author_icon_url: Some(
                                            self.music_manager.spinning_disk.clone()
                                        ),
                                        title: Some(
                                            self.metadata.title
                                                .as_ref()
                                                .unwrap_or(&"<UNKNOWN>".to_string())
                                                .to_string()
                                        ),
                                        url: Some(self.url.to_string()),
                                        thumbnail: if let Some(url) = &self.metadata.thumbnail {
                                            Some(url.to_string())
                                        } else {
                                            None
                                        },
                                        fields: Some(
                                            vec![DiscordEmbedField {
                                                name: format!("Duration"),
                                                value: if
                                                    let Some(duration) =
                                                        self.metadata.duration.as_ref()
                                                {
                                                    format_duration(duration)
                                                } else {
                                                    format!("<Unknown duration>")
                                                },
                                                inline: true,
                                            }]
                                        ),
                                        footer_text: Some(
                                            format!("Requested by @{}", self.requested_by.name)
                                        ),
                                        footer_icon_url: self.requested_by.avatar.map(|hash|
                                            cdn_avatar!(self.requested_by.id, hash)
                                        ),
                                        color: Some(ColorResolvables::Green.as_u32()),
                                        ..Default::default()
                                    })
                                ]
                            )
                        {
                            let _ = message.await;
                        }
                    } else if track_state.playing == PlayMode::End {
                        // if the Player's looping state is queue, add the url of just finished track to the end of the waiting queue
                        match self.music_manager.get_loop_state(self.guild_id) {
                            PlayerLoopState::LoopQueue => {
                                // Re-add the current song to the end of the queue
                                self.music_manager.extend_waiting_queue(
                                    self.guild_id,
                                    &vec![self.url.clone()]
                                );
                            }
                            _ => {
                                // Handle other states or no loop
                            }
                        }
                        // When a track ends, check for the next URL in waiting_track_urls
                        self.music_manager.set_current_song(self.guild_id, None);
                        if
                            let Some(next_url) = self.music_manager.pop_next_track_url(
                                self.guild_id
                            )
                        {
                            // Add next track to the queue
                            if
                                let Err(e) = add_track_to_queue(
                                    Arc::clone(&self.http),
                                    Arc::clone(&self.music_manager),
                                    self.channel_id,
                                    self.guild_id,
                                    &self.requested_by,
                                    next_url
                                ).await
                            {
                                eprintln!("Error adding next track to queue: {:?}", e);
                            }
                        } else {
                            if
                                let Ok(message) = self.http.create_message(self.channel_id).embeds(
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
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        None
    }
}

pub async fn add_track_to_queue(
    http: Arc<HttpClient>,
    music_manager: Arc<VoiceManager>,
    channel_id: Id<ChannelMarker>,
    guild_id: Id<GuildMarker>,
    requested_by: &User,
    url: String
) -> Result<(), Box<dyn Error + Sync + Send>> {
    // Create a new source from the URL
    let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());

    // Fetch metadata for the new track
    match source.aux_metadata().await {
        Ok(metadata) => {
            if let Some(call_lock) = music_manager.songbird.get(guild_id) {
                // Add the source to the track queue
                let mut call = call_lock.lock().await;

                // We need to ensure that this guild has a TrackQueue created for it.
                let track_queue = {
                    let mut queues = music_manager.trackqueues.write().unwrap();
                    queues.entry(guild_id).or_default().clone()
                };
                let handle = track_queue.add_source(source.into(), &mut *call).await;

                let start_event_handler = MusicEventHandler {
                    http: Arc::clone(&http),
                    channel_id,
                    music_manager: Arc::clone(&music_manager),
                    guild_id,
                    url: url.to_string(),
                    metadata: metadata.clone(),
                    requested_by: requested_by.clone(),
                };

                handle
                    .add_event(Event::Track(TrackEvent::Play), start_event_handler)
                    .expect("Failed to add event handler");

                let end_event_handler = MusicEventHandler {
                    http: Arc::clone(&http),
                    channel_id,
                    music_manager: Arc::clone(&music_manager),
                    guild_id,
                    url: url.to_string(),
                    metadata: metadata.clone(),
                    requested_by: requested_by.clone(),
                };
                handle
                    .add_event(Event::Track(TrackEvent::End), end_event_handler)
                    .expect("Failed to add event handler");
            }
        }
        Err(e) => {
            println!("Error retrieving metadata: {:?}", e);
            http.create_message(channel_id).content("Error retrieving track information.")?.await?;
        }
    }

    Ok(())
}
