use std::error::Error;
use fluent_bundle::FluentArgs;
use futures_util::{ stream::StreamExt, future::join_all };
use async_trait::async_trait;
use songbird::input::{ YoutubeDl, Compose };
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    channel::message::{ Component, component::{ ActionRow, Button }, Embed, MessageFlags },
    application::interaction::{ Interaction, InteractionData },
    http::interaction::{ InteractionResponseType, InteractionResponse },
};
use tokio::time::{ timeout, Duration };

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    utilities::{ utils::{ ColorResolvables, color_to_button_style }, generate_random_string },
};
pub struct QueueCommand {}

#[async_trait]
impl ContextCommand for QueueCommand {
    fn name(&self) -> &'static str {
        "queue"
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let queued_urls = client.voice_music_manager.get_waiting_queue(guild_id);
        if queued_urls.len() == 0 {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some(
                            client.get_locale_string(&config.locale, "music-queue-empty", None)
                        ),
                        color: Some(ColorResolvables::Yellow.as_u32()),
                        ..Default::default()
                    }]
                )
            ).await;
            return Ok(());
        }

        let args = FluentArgs::from_iter(
            vec![("count", queued_urls.len().to_string()), ("username", msg.author.name.clone())]
        );

        let mut embed = DiscordEmbed {
            author_name: Some(
                client.get_locale_string(&config.locale, "music-queue-title", Some(&args))
            ),
            author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
            color: Some(ColorResolvables::Blue.as_u32()),
            ..Default::default()
        };

        if let Some((metadata, _)) = client.voice_music_manager.get_current_song(guild_id) {
            embed.title = Some(
                format!("Playing: {}", metadata.title.unwrap_or(format!("Unknown")))
            );
            embed.thumbnail = metadata.thumbnail;
        }

        let sent_msg = client
            .reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(vec![embed.clone()])
            ).await?
            .model().await?;

        async fn get_one_url(url: &str) -> String {
            let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());
            match source.aux_metadata().await {
                Ok(metadata) => {
                    format!(
                        "[{}]({})",
                        metadata.title.unwrap_or_else(|| "Unknown title".to_string()),
                        url
                    )
                }
                Err(_) => { format!("[Unknown track]({})", url) }
            }
        }

        async fn get_list(
            queued_urls: &[String],
            page_number: usize
        ) -> Result<String, Box<dyn Error + Send + Sync>> {
            let start_index = page_number * 10;
            let end_index = (page_number * 10 + 10).min(queued_urls.len());

            let tasks = queued_urls[start_index..end_index]
                .iter()
                .map(|url| {
                    let url_cloned = url.clone();
                    tokio::spawn(async move { get_one_url(&url_cloned).await })
                })
                .collect::<Vec<_>>();

            let results = join_all(tasks).await
                .into_iter()
                .map(|result| result.unwrap_or_else(|_| "Error fetching data".to_string()))
                .collect::<Vec<_>>();

            let list = results
                .iter()
                .enumerate()
                .map(|(i, song)| format!("{}. {}", start_index + i + 1, song))
                .collect::<Vec<String>>()
                .join("\n");

            Ok(list)
        }

        fn make_components(buttons: &Vec<Button>) -> Vec<Component> {
            buttons
                .iter()
                .map(|button| Component::Button(button.clone()))
                .collect::<Vec<Component>>()
        }

        let mut current_page: usize = 0;
        let list = get_list(&queued_urls, 0).await?;

        let pages = ((queued_urls.len() as f64) / (10 as f64)).ceil() as usize;

        embed.description = Some(list);

        if pages > 1 {
            let unique_key = generate_random_string(10);
            // Unique IDs for buttons
            let prev_button_id = format!("prev_page:{}", unique_key);
            let next_button_id = format!("next_page:{}", unique_key);

            let mut buttons = vec![
                Button {
                    custom_id: Some(prev_button_id.clone()),
                    disabled: true,
                    emoji: None,
                    label: Some(client.get_locale_string(&config.locale, "music-queue-prev", None)),
                    style: color_to_button_style("blue"),
                    url: None,
                },
                Button {
                    custom_id: Some(next_button_id.clone()),
                    disabled: false,
                    emoji: None,
                    label: Some(client.get_locale_string(&config.locale, "music-queue-next", None)),
                    style: color_to_button_style("blue"),
                    url: None,
                }
            ];

            let _ = client.http
                .update_message(sent_msg.channel_id, sent_msg.id)
                .embeds(Some(&vec![Embed::from(embed.clone())]))?
                .components(
                    Some(
                        &vec![
                            Component::ActionRow(ActionRow {
                                components: make_components(&buttons),
                            })
                        ]
                    )
                )?.await?;

            // Set the timeout duration to 5 minutes
            let timeout_duration = Duration::from_secs(5 * 60);

            let mut components = client.standby.wait_for_component_stream(
                sent_msg.id,
                |event: &Interaction| {
                    if let Some(InteractionData::MessageComponent(_)) = &event.data {
                        true
                    } else {
                        false
                    }
                }
            );

            let result: Result<
                Result<(), Box<dyn Error + Send + Sync>>,
                tokio::time::error::Elapsed
            > = timeout(timeout_duration, async {
                while let Some(button_interaction) = components.next().await {
                    if let Some(InteractionData::MessageComponent(data)) = &button_interaction.data {
                        if data.custom_id.ends_with(&unique_key) {
                            if button_interaction.author_id().unwrap() == msg.author.id {
                                // Acknowledge the interaction
                                let _ = client.http
                                    .interaction(button_interaction.application_id)
                                    .create_response(
                                        button_interaction.id,
                                        &button_interaction.token,
                                        &(InteractionResponse {
                                            kind: InteractionResponseType::DeferredUpdateMessage,
                                            data: None,
                                        })
                                    ).await?;

                                if data.custom_id == next_button_id {
                                    current_page += 1;
                                } else {
                                    current_page -= 1;
                                }
                                if current_page == pages - 1 {
                                    buttons[0].disabled = false;
                                    buttons[1].disabled = true;
                                } else if current_page == 0 {
                                    buttons[0].disabled = true;
                                    buttons[1].disabled = false;
                                } else {
                                    buttons[0].disabled = false;
                                    buttons[1].disabled = false;
                                }
                                embed.description = Some(
                                    get_list(&queued_urls, current_page).await?
                                );
                                let _ = client.http
                                    .update_message(sent_msg.channel_id, sent_msg.id)
                                    .embeds(Some(&vec![Embed::from(embed.clone())]))?
                                    .components(
                                        Some(
                                            &vec![
                                                Component::ActionRow(ActionRow {
                                                    components: make_components(&buttons),
                                                })
                                            ]
                                        )
                                    )?.await;
                            } else {
                                let _ = client.http
                                    .interaction(button_interaction.application_id)
                                    .create_followup(&button_interaction.token)
                                    .embeds(
                                        &vec![
                                            Embed::from(DiscordEmbed {
                                                description: Some(
                                                    client.get_locale_string(
                                                        &config.locale,
                                                        "interaction-denied",
                                                        None
                                                    )
                                                ),
                                                color: Some(ColorResolvables::Red.as_u32()),
                                                ..Default::default()
                                            })
                                        ]
                                    )?
                                    .flags(MessageFlags::EPHEMERAL).await;
                            }
                        }
                    }
                }
                Ok::<(), Box<dyn Error + Send + Sync>>(())
            }).await;

            match result {
                Ok(_) => {}
                Err(_) => {
                    buttons[0].disabled = true;
                    buttons[1].disabled = true;
                    let _ = client.http.update_message(sent_msg.channel_id, sent_msg.id).components(
                        Some(
                            &vec![
                                Component::ActionRow(ActionRow {
                                    components: make_components(&buttons),
                                })
                            ]
                        )
                    )?.await;
                }
            }
        } else {
            let _ = client.edit_message(
                sent_msg.channel_id,
                sent_msg.id,
                MessageContent::DiscordEmbeds(vec![embed.clone()])
            ).await;
        }

        Ok(())
    }
}
