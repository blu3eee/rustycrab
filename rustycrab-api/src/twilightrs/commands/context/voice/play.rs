use std::{ error::Error, sync::Arc };

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use songbird::input::{ YoutubeDl, Compose };

use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_model::id::Id;
use twilight_model::id::marker::GuildMarker;

use crate::twilightrs::bot::voice_music::parse_url::parse_url_or_search_query;

use crate::utilities::app_error::BoxedError;
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
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        // get the voice channel of the command author
        let channel_id = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => { state.channel_id() }
            None => {
                return Err(
                    client.get_locale_string(&config.locale, "music-user-novoice", None).into()
                );
            }
        };

        // get the current call of the bot (if there is any)
        // if the bot is already in a channel, check if the bot is in the same channel as the command author
        // if the bot is not in a voice channel, ask the bot to join the voice channel of the command author
        // return error if the bot is not in the same channel or not able to connect
        if let Ok(call_lock) = client.fetch_call_lock(guild_id, Some(&config.locale)).await {
            let mut call = call_lock.lock().await;
            let _ = call.deafen(true).await;
            client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;
        } else if let Err(_) = client.voice_music_manager.songbird.join(guild_id, channel_id).await {
            return Err(
                client.get_locale_string(&config.locale, "music-cannot-connect", None).into()
            );
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
        let (valid_urls, _) = if let Some(ParsedArg::Text(arg)) = command_args.first() {
            match parse_url_or_search_query(&client, &config.locale, arg).await {
                Ok(urls) => { urls }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                    return Err(
                        client
                            .get_locale_string(&config.locale, "command-play-invalid-url", None)
                            .into()
                    );
                }
            }
        } else {
            return Err(
                client.get_locale_string(&config.locale, "command-play-invalid-url", None).into()
            );
        };

        // if the parsed urls is empty, return error
        if valid_urls.is_empty() {
            return Err(
                client.get_locale_string(&config.locale, "command-play-invalid-url", None).into()
            );
        }

        let _ = build_response(&client, msg, valid_urls, guild_id, config).await;

        let _ = client.http.delete_message(sent_msg.channel_id, sent_msg.id).await;

        Ok(())
    }
}

impl PlayCommand {}

async fn build_response(
    client: &DiscordClient,
    msg: &MessageCreate,
    urls: Vec<String>,
    guild_id: Id<GuildMarker>,
    config: &GuildConfigModel
) -> Result<(), BoxedError> {
    // We need to ensure that this guild has a TrackQueue created for it.
    let track_queue = client.voice_music_manager.get_play_queue(guild_id);

    let embed = if track_queue.is_empty() {
        // Add tracks to the queue, counting successfully added ones
        let _ = add_track_to_queue(
            Arc::clone(&client),
            Arc::clone(&client.voice_music_manager),
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
        let _ = client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(vec![embed])
        ).await;
    }

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
