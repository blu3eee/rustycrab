use std::time::Duration;
use std::{ error::Error, sync::Arc };

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use songbird::input::{ YoutubeDl, Compose };

use spotify::models::SpotifyPlaylistResponse;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_model::id::Id;
use twilight_model::id::marker::GuildMarker;

use crate::utilities::app_error::BoxedError;
use crate::utilities::format_duration;
use crate::{
    twilightrs::{
        messages::DiscordEmbedField,
        commands::context::{
            context_command::{ ContextCommand, GuildConfigModel },
            ParsedArg,
            ArgSpec,
            ArgType,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
        bot::voice_music::{
            utils::parse_url::parse_url_or_search_query,
            player::{ track_info::track_info_fields, add_track_to_queue::add_track_to_queue },
        },
    },
    cdn_avatar,
};

pub struct PlayCommand {}

#[async_trait]
impl ContextCommand for PlayCommand {
    fn name(&self) -> &'static str {
        "play"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("url/search", ArgType::Text, false)]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // make sure the command is in a guild
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        // get the voice channel of the command author
        let channel_id = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => { state.channel_id() }
            None => {
                return Err("music-user-novoice".into());
            }
        };

        if let Some(_) = client.get_bot_vc_channel_id(guild_id).await? {
            client.verify_same_voicechannel(guild_id, msg.author.id).await?;
        }

        match client.voice_music_manager.songbird.join(guild_id, channel_id).await {
            Ok(call_lock) => {
                let mut call = call_lock.lock().await;
                let _ = call.deafen(true).await;
            }
            Err(e) => {
                println!("error joining channel {e}");
                return Err(
                    client.get_locale_string(&config.locale, "music-cannot-connect", None).into()
                );
            }
        }

        let sent_msg = client
            .reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some(
                            client.get_locale_string(&config.locale, "music-loading-url", None)
                        ),
                        color: Some(ColorResolvables::Yellow.as_u32()),
                        ..Default::default()
                    }]
                )
            ).await?
            .model().await?;

        // now the bot has connect to the channel
        // get the url(s) to play audio
        let (valid_urls, playlist_data) = if let Some(ParsedArg::Text(arg)) = command_args.first() {
            match parse_url_or_search_query(&client, &config.locale, arg).await {
                Ok(urls) => { urls }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                    return Err("command-play-invalid-url".into());
                }
            }
        } else {
            return Err("command-play-invalid-url".into());
        };

        // if the parsed urls is empty, return error
        if valid_urls.is_empty() {
            return Err("command-play-invalid-url".into());
        }

        let _ = build_response(&client, config, msg, guild_id, valid_urls, playlist_data).await;

        let _ = client.http.delete_message(sent_msg.channel_id, sent_msg.id).await;

        Ok(())
    }
}

impl PlayCommand {}

async fn build_response(
    client: &DiscordClient,
    config: &GuildConfigModel,
    msg: &MessageCreate,
    guild_id: Id<GuildMarker>,
    urls: Vec<String>,
    playlist_data: Option<SpotifyPlaylistResponse>
) -> Result<(), BoxedError> {
    // We need to ensure that this guild has a TrackQueue created for it.
    let track_queue = client.voice_music_manager.get_play_queue(guild_id);

    let mut embeds: Vec<DiscordEmbed> = Vec::new();

    // spotify playlist data
    if let Some(playlist_data) = playlist_data {
        embeds.push(DiscordEmbed {
            author_name: Some(
                client.get_locale_string(&config.locale, "music-playlist-found", None)
            ),
            author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
            fields: Some(
                vec![
                    DiscordEmbedField {
                        name: client.get_locale_string(
                            &config.locale,
                            "music-content-credits-spotify",
                            None
                        ),
                        value: format!(
                            "[{}]({})",
                            playlist_data.name,
                            playlist_data.external_urls.spotify
                        ),
                        inline: false,
                    },
                    DiscordEmbedField {
                        name: client.get_locale_string(&config.locale, "music-duration", None),
                        value: format!(
                            "{}",
                            format_duration(
                                &Duration::from_millis(
                                    playlist_data.tracks.items
                                        .iter()
                                        .map(|track_item| track_item.track.duration_ms)
                                        .sum::<u64>()
                                )
                            )
                        ),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: format!("Tracks"),
                        value: format!("{}", playlist_data.tracks.total),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: format!("Playlist creator"),
                        value: format!(
                            "[{}]({})",
                            playlist_data.owner.display_name.unwrap_or(
                                playlist_data.owner.id.to_string()
                            ),
                            playlist_data.owner.external_urls.spotify
                        ),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: format!("Followers"),
                        value: format!("{}", playlist_data.followers.total),
                        inline: true,
                    }
                ]
            ),
            thumbnail: if playlist_data.images.len() > 0 {
                Some(playlist_data.images[0].url.clone())
            } else {
                None
            },
            description: playlist_data.description,
            color: Some(ColorResolvables::SpotifyGreen.as_u32()),
            ..Default::default()
        });
    }

    let embed = if track_queue.is_empty() {
        // Add tracks to the queue, counting successfully added ones
        let _ = add_track_to_queue(
            Arc::clone(&client),
            msg.channel_id,
            guild_id.clone(),
            &msg.author,
            urls[0].clone()
        ).await.is_ok();

        client.voice_music_manager.extend_waiting_queue(guild_id, &urls[1..].to_vec());
        create_embed(&client, &config, urls.len() - 1, &urls[0]).await?
    } else {
        client.voice_music_manager.extend_waiting_queue(guild_id, &urls);
        create_embed(&client, &config, urls.len(), &urls[0]).await?
    };

    if let Some(mut embed) = embed {
        embed.footer_text = Some(
            client.get_locale_string(
                &config.locale,
                "requested-user",
                Some(&FluentArgs::from_iter(vec![("username", msg.author.name.clone())]))
            )
        );
        embed.footer_icon_url = msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash));
        embed.author_icon_url = Some(client.voice_music_manager.spinning_disk.clone());
        embeds.push(embed);
    }

    let _ = client.reply_message(
        msg.channel_id,
        msg.id,
        MessageContent::DiscordEmbeds(embeds)
    ).await;

    Ok(())
}

async fn create_embed(
    client: &DiscordClient,
    config: &GuildConfigModel,
    waiting_tracks: usize,
    first_track_url: &str
) -> Result<Option<DiscordEmbed>, BoxedError> {
    if waiting_tracks == 0 {
        return Ok(None);
    }

    let mut args = FluentArgs::new();
    args.set("count", waiting_tracks);
    let key = if waiting_tracks > 1 {
        "command-play-added-tracks"
    } else {
        "command-play-added-track"
    };

    let embed_content = if waiting_tracks == 1 {
        let metadata = YoutubeDl::new(reqwest::Client::new(), first_track_url.to_string())
            .aux_metadata().await
            .map_err(|e| {
                println!("Error retrieving metadata: {:?}", e);
                client.get_locale_string(&config.locale, "music-error-track", None)
            })?;
        Some(track_info_fields(&client, &config.locale, &metadata, Some(1)))
    } else {
        None
    };

    Ok(
        Some(DiscordEmbed {
            author_name: Some(client.get_locale_string(&config.locale, key, Some(&args))),
            color: Some(ColorResolvables::Green.as_u32()),
            fields: embed_content,
            ..Default::default()
        })
    )
}
