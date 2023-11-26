use std::{ error::Error, sync::Arc };

use async_trait::async_trait;
use songbird::{ input::{ YoutubeDl, Compose }, Event, TrackEvent };

use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{ marker::{ GuildMarker, ChannelMarker }, Id },
    user::User,
};

use crate::{
    twilightrs::{
        commands::context::{
            context_command::{ ContextCommand, GuildConfigModel },
            ParsedArg,
            ArgSpec,
            ArgType,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::{ DiscordEmbed, DiscordEmbedField },
        bot::youtube_api::{ search_youtube, is_youtube_playlist_url, fetch_playlist_videos },
    },
    utilities::{ format_duration, utils::ColorResolvables },
    cdn_avatar,
};

use super::song_bird_event_handler::MusicEventHandler;

pub struct PlayCommand {}

#[async_trait]
impl ContextCommand for PlayCommand {
    fn name(&self) -> &'static str {
        "play"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["music"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("url", ArgType::Text, false)]
    }

    async fn run(
        &self,
        client: DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // Ensure the user is in a voice channel
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => {
                return Ok(());
            } // Command not used in a guild
        };

        let channel_id = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => state.channel_id(),
            None => {
                // Notify user they need to be in a voice channel
                client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::Text(
                        "You need to be in a voice channel to use the command".to_string()
                    )
                ).await?;
                return Ok(());
            }
        };

        match client.songbird.join(guild_id, channel_id).await {
            Ok(_) => {}
            Err(_) => {
                // Notify user they need to be in a voice channel
                client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::Text("I can't connect to your channel".to_string())
                ).await?;
                return Ok(());
            }
        }

        if let Some(ParsedArg::Text(arg)) = command_args.first() {
            // Check if arg is a URL or a search query
            let urls: Vec<String> = if arg.starts_with("http://") || arg.starts_with("https://") {
                if is_youtube_playlist_url(arg) {
                    fetch_playlist_videos(arg).await?
                } else {
                    vec![arg.clone()] // Use the URL as is
                }
            } else {
                // Perform a YouTube search and get the first result's URL
                match search_youtube(arg).await {
                    Ok(url) => { vec![url] }
                    Err(e) => {
                        eprintln!("{}", e);
                        return Ok(());
                    }
                }
            };

            if !urls.is_empty() {
                // We need to ensure that this guild has a TrackQueue created for it.
                let track_queue = {
                    let mut queues = client.trackqueues.write().unwrap();
                    queues.entry(guild_id).or_default().clone()
                };

                if track_queue.is_empty() {
                    let first_url = &urls[0];
                    let remaining_urls = urls.clone().into_iter().skip(1).collect::<Vec<_>>();
                    {
                        let mut waiting_urls = client.waiting_track_urls.write().unwrap();
                        waiting_urls.entry(guild_id).or_default().extend(remaining_urls);
                    }
                    if urls.len() > 1 {
                        let _ = client.reply_message(
                            msg.channel_id,
                            msg.id,
                            MessageContent::DiscordEmbeds(
                                vec![DiscordEmbed {
                                    author_name: Some(
                                        format!("Added {} tracks to queue", urls.len())
                                    ),
                                    author_icon_url: Some(
                                        "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
                                    ),
                                    footer_text: Some(format!("Requested by @{}", msg.author.name)),
                                    footer_icon_url: msg.author.avatar.map(|hash|
                                        cdn_avatar!(msg.author.id, hash)
                                    ),
                                    color: Some(ColorResolvables::Green.as_u32()),
                                    ..Default::default()
                                }]
                            )
                        ).await?;
                    }
                    let _ = add_track_to_queue(
                        Arc::clone(&client),
                        msg.channel_id,
                        guild_id.clone(),
                        &msg.author,
                        first_url.clone()
                    ).await;
                } else {
                    {
                        let mut waiting_urls = client.waiting_track_urls.write().unwrap();
                        waiting_urls.entry(guild_id).or_default().extend(urls.clone());
                    }

                    let message_content = if urls.len() > 1 {
                        MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                author_name: Some(format!("Added {} tracks to queue", urls.len())),
                                author_icon_url: Some(
                                    "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
                                ),
                                footer_text: Some(format!("Requested by @{}", msg.author.name)),
                                footer_icon_url: msg.author.avatar.map(|hash|
                                    cdn_avatar!(msg.author.id, hash)
                                ),
                                color: Some(ColorResolvables::Green.as_u32()),
                                ..Default::default()
                            }]
                        )
                    } else {
                        let url = &urls[0];
                        let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());
                        // Fetch metadata for the new track
                        match source.aux_metadata().await {
                            Ok(metadata) => {
                                let position = {
                                    let mut waiting_urls = client.waiting_track_urls
                                        .write()
                                        .unwrap();
                                    waiting_urls.entry(guild_id).or_default().len()
                                };
                                MessageContent::DiscordEmbeds(
                                    vec![DiscordEmbed {
                                        author_name: Some("Added track".to_string()),
                                        author_icon_url: Some(
                                            "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
                                        ),
                                        thumbnail: if let Some(turl) = &metadata.thumbnail {
                                            Some(turl.to_string())
                                        } else {
                                            None
                                        },
                                        fields: Some(
                                            vec![
                                                DiscordEmbedField {
                                                    name: format!("Track"),
                                                    value: format!(
                                                        "[**{}**]({})",
                                                        metadata.title
                                                            .as_ref()
                                                            .unwrap_or(&"<UNKNOWN>".to_string())
                                                            .to_string(),
                                                        url.to_string()
                                                    ),
                                                    inline: false,
                                                },
                                                DiscordEmbedField {
                                                    name: format!("Duration"),
                                                    value: if
                                                        let Some(duration) =
                                                            metadata.duration.as_ref()
                                                    {
                                                        format_duration(duration)
                                                    } else {
                                                        format!("<Unknown duration>")
                                                    },
                                                    inline: true,
                                                },
                                                DiscordEmbedField {
                                                    name: format!("Position in queue"),
                                                    value: position.to_string(),
                                                    inline: true,
                                                }
                                            ]
                                        ),
                                        footer_text: Some(
                                            format!("Requested by @{}", msg.author.name)
                                        ),
                                        footer_icon_url: msg.author.avatar.map(|hash|
                                            cdn_avatar!(msg.author.id, hash)
                                        ),
                                        color: Some(ColorResolvables::Green.as_u32()),
                                        ..Default::default()
                                    }]
                                )
                            }
                            Err(e) => {
                                println!("Error retrieving metadata: {:?}", e);
                                MessageContent::Text(format!("Error retrieving track information."))
                            }
                        }
                    };

                    let _ = client.reply_message(msg.channel_id, msg.id, message_content).await;
                }
            }
        } else {
            client.http
                .create_message(msg.channel_id)
                .content("Please provide a valid URL or search query to play.")?.await?;
        }

        Ok(())
    }
}

impl PlayCommand {}

pub async fn add_track_to_queue(
    client: DiscordClient,
    channel_id: Id<ChannelMarker>,
    guild_id: Id<GuildMarker>,
    requested_by: &User,
    url: String
) -> Result<(), Box<dyn Error>> {
    // Create a new source from the URL
    let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());

    // Fetch metadata for the new track
    match source.aux_metadata().await {
        Ok(metadata) => {
            if let Some(call_lock) = client.songbird.get(guild_id) {
                // Add the source to the track queue
                let mut call = call_lock.lock().await;

                // We need to ensure that this guild has a TrackQueue created for it.
                let track_queue = {
                    let mut queues = client.trackqueues.write().unwrap();
                    queues.entry(guild_id).or_default().clone()
                };
                let handle = track_queue.add_source(source.into(), &mut *call).await;

                let start_event_handler = MusicEventHandler {
                    client: Arc::clone(&client),
                    channel_id,
                    guild_id,
                    url: url.to_string(),
                    metadata: metadata.clone(),
                    requested_by: requested_by.clone(),
                };

                handle
                    .add_event(Event::Track(TrackEvent::Play), start_event_handler)
                    .expect("Failed to add event handler");

                let end_event_handler = MusicEventHandler {
                    client: Arc::clone(&client),
                    channel_id,
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
            client.http
                .create_message(channel_id)
                .content("Error retrieving track information.")?.await?;
        }
    }

    Ok(())
}
