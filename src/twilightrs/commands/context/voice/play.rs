use std::{ error::Error, sync::Arc };

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use songbird::input::{ YoutubeDl, Compose };

use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::bot::voice_music::parse_url::parse_url_or_search_query;

use crate::{
    twilightrs::{
        commands::context::{
            context_command::{ ContextCommand, GuildConfigModel },
            ParsedArg,
            ArgSpec,
            ArgType,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
        bot::voice_music::voice_manager::{ add_track_to_queue, track_info_fields },
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
};

pub struct PlayCommand {}

#[async_trait]
impl ContextCommand for PlayCommand {
    fn name(&self) -> &'static str {
        "play"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("url", ArgType::Text, false)]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let channel_id = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => state.channel_id(),
            None => {
                return Err(
                    client.get_locale_string(&config.locale, "music-user-novoice", None).into()
                );
            }
        };

        if let Err(_) = client.voice_music_manager.songbird.join(guild_id, channel_id).await {
            return Err(
                client.get_locale_string(&config.locale, "music-cannot-connect", None).into()
            );
        }

        let urls = if let Some(ParsedArg::Text(arg)) = command_args.first() {
            parse_url_or_search_query(&client, &config.locale, arg).await?
        } else {
            return Err(
                client.get_locale_string(&config.locale, "command-play-invalid-url", None).into()
            );
        };

        if urls.is_empty() {
            return Err(
                client.get_locale_string(&config.locale, "command-play-invalid-url", None).into()
            );
        }

        // We need to ensure that this guild has a TrackQueue created for it.
        let track_queue = client.voice_music_manager.get_play_queue(guild_id);

        let mut args = FluentArgs::new();
        args.set("count", urls.len());
        let embed = if track_queue.is_empty() {
            let first_url = &urls[0];
            let remaining_urls = urls.clone().into_iter().skip(1).collect::<Vec<_>>();
            client.voice_music_manager.extend_waiting_queue(guild_id, &remaining_urls);
            let _ = add_track_to_queue(
                Arc::clone(&client),
                Arc::clone(&client.voice_music_manager),
                msg.channel_id,
                guild_id.clone(),
                &msg.author,
                first_url.clone()
            ).await;
            if urls.len() > 1 {
                Some(DiscordEmbed {
                    author_name: Some(
                        client.get_locale_string(
                            &config.locale,
                            "command-play-added-tracks",
                            Some(&args)
                        )
                    ),
                    color: Some(ColorResolvables::Green.as_u32()),
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            {
                let mut waiting_urls = client.voice_music_manager.waiting_track_urls
                    .write()
                    .unwrap();
                waiting_urls.entry(guild_id).or_default().extend(urls.clone());
            }

            if urls.len() > 1 {
                Some(DiscordEmbed {
                    author_name: Some(
                        client.get_locale_string(
                            &config.locale,
                            "command-play-added-tracks",
                            Some(&args)
                        )
                    ),
                    color: Some(ColorResolvables::Green.as_u32()),
                    ..Default::default()
                })
            } else {
                let url = &urls[0];
                let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());
                // Fetch metadata for the new track
                let embed = match source.aux_metadata().await {
                    Ok(metadata) => {
                        let position = {
                            let mut waiting_urls = client.voice_music_manager.waiting_track_urls
                                .write()
                                .unwrap();
                            waiting_urls.entry(guild_id).or_default().len()
                        };
                        DiscordEmbed {
                            author_name: Some(
                                client.get_locale_string(
                                    &config.locale,
                                    "command-play-added-track",
                                    None
                                )
                            ),
                            thumbnail: metadata.thumbnail.clone(),
                            fields: Some(
                                track_info_fields(
                                    &client,
                                    &config.locale,
                                    &metadata,
                                    Some(position)
                                )
                            ),
                            color: Some(ColorResolvables::Green.as_u32()),
                            ..Default::default()
                        }
                    }
                    Err(e) => {
                        println!("Error retrieving metadata: {:?}", e);

                        return Err(
                            client
                                .get_locale_string(&config.locale, "music-error-track", None)
                                .into()
                        );
                    }
                };

                Some(embed)
            }
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
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(vec![embed])
            ).await;
        }

        Ok(())
    }
}

impl PlayCommand {}
