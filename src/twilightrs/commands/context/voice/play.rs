use std::error::Error;

use async_trait::async_trait;
use songbird::input::{ YoutubeDl, Compose };

use twilight_model::gateway::payload::incoming::MessageCreate;

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
    },
    utilities::format_duration,
};

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

        if let Some(ParsedArg::Text(url)) = command_args.first() {
            let mut src = YoutubeDl::new(reqwest::Client::new(), url.clone());
            match src.aux_metadata().await {
                Ok(metadata) => {
                    if let Some(call_lock) = client.songbird.get(guild_id) {
                        let mut call = call_lock.lock().await;

                        let handle = call.play_input(src.into());

                        client.reply_message(
                            msg.channel_id,
                            msg.id,
                            MessageContent::DiscordEmbeds(
                                vec![DiscordEmbed {
                                    author_name: Some("Now playing".to_string()),
                                    author_icon_url: Some(
                                        "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
                                    ),
                                    title: Some(
                                        metadata.title
                                            .as_ref()
                                            .unwrap_or(&"<UNKNOWN>".to_string())
                                            .to_string()
                                    ),
                                    url: Some(url.to_string()),
                                    thumbnail: if let Some(url) = metadata.thumbnail {
                                        Some(url)
                                    } else {
                                        None
                                    },
                                    fields: Some(
                                        vec![
                                            DiscordEmbedField {
                                                name: format!("Requested by"),
                                                value: format!("<@{}>", msg.author.id),
                                                inline: true,
                                            },
                                            DiscordEmbedField {
                                                name: format!("Duration"),
                                                value: if
                                                    let Some(duration) = metadata.duration.as_ref()
                                                {
                                                    format_duration(duration)
                                                } else {
                                                    format!("<Unknown duration>")
                                                },
                                                inline: true,
                                            }
                                        ]
                                    ),
                                    ..Default::default()
                                }]
                            )
                        ).await?;

                        let mut store = client.trackdata.write().unwrap();
                        store.insert(guild_id, handle);
                    } else {
                        println!("Could not get call lock for Songbird");
                    }
                }
                Err(e) => {
                    println!("Error retrieving metadata: {:?}", e);
                    client.http
                        .create_message(msg.channel_id)
                        .content("Error retrieving track information.")?.await?;
                }
            }
        } else {
            client.http
                .create_message(msg.channel_id)
                .content("Please provide a URL to play.")?.await?;
        }
        Ok(())
    }
}

impl PlayCommand {}
