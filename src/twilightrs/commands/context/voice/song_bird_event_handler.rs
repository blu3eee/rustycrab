use std::sync::Arc;

use songbird::{
    EventHandler as SongbirdEventHandler,
    Event,
    EventContext,
    input::AuxMetadata,
    tracks::PlayMode,
};
use async_trait::async_trait;
use twilight_model::{ id::{ Id, marker::{ ChannelMarker, GuildMarker } }, user::User };

use crate::{
    twilightrs::{
        discord_client::{ DiscordClient, MessageContent },
        messages::{ DiscordEmbedField, DiscordEmbed },
    },
    cdn_avatar,
    utilities::{ utils::ColorResolvables, format_duration },
};

use super::play::add_track_to_queue;

pub struct MusicEventHandler {
    pub client: DiscordClient,
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
                    println!("event handler MusicEventHandler {:?}", track_state);
                    // Logic to execute when a track starts playing.
                    // You can check specific conditions here, for example:
                    if track_state.playing == PlayMode::Play {
                        let guild = if let Ok(guild) = self.client.get_guild(self.guild_id).await {
                            guild.name
                        } else {
                            self.guild_id.to_string()
                        };

                        println!(
                            "[Guild: {}] Track started playing: {:?}",
                            guild,
                            self.metadata.title.clone().unwrap_or(format!("unknown"))
                        );

                        let _ = self.client.send_message(
                            self.channel_id,
                            MessageContent::DiscordEmbeds(
                                vec![DiscordEmbed {
                                    author_name: Some("Now playing".to_string()),
                                    author_icon_url: Some(
                                        "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
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
                                                let Some(duration) = self.metadata.duration.as_ref()
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
                                }]
                            )
                        ).await;
                    } else if track_state.playing == PlayMode::End {
                        // When a track ends, check for the next URL in waiting_track_urls
                        if let Some(next_url) = self.client.pop_next_track_url(self.guild_id).await {
                            // Add next track to the queue
                            if
                                let Err(e) = add_track_to_queue(
                                    Arc::clone(&self.client),
                                    self.channel_id,
                                    self.guild_id,
                                    &self.requested_by,
                                    next_url
                                ).await
                            {
                                eprintln!("Error adding next track to queue: {:?}", e);
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
