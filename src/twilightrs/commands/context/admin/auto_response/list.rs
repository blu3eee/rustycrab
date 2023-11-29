use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use futures_util::StreamExt;
use tokio::time::timeout;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    application::interaction::{ InteractionData, Interaction },
    channel::message::{ component::{ Button, ActionRow }, Embed, MessageFlags, Component },
    http::interaction::{ InteractionResponse, InteractionResponseType },
};
use std::{ error::Error, time::Duration };

use crate::{
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, context_command::GuildConfigModel },
        discord_client::{ DiscordClient, MessageContent },
        utils::{ reply_command, make_components },
        messages::DiscordEmbed,
    },
    queries::auto_responses_queries::AutoResponsesQueries,
    utilities::{ utils::{ ColorResolvables, color_to_button_style }, generate_random_string },
    multi_bot_guild_entities_queries::MultipleBotGuildEntityQueries,
};

use super::AutoResCommand;
pub struct ListAutoResponseCommand;

#[async_trait]
impl ContextCommand for ListAutoResponseCommand {
    fn name(&self) -> &'static str {
        "list"
    }

    fn parent_command(&self) -> Option<Box<dyn ContextCommand>> {
        Some(Box::new(AutoResCommand {}) as Box<dyn ContextCommand>)
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

        let bot = client.get_bot().await?;

        let bot_discord_id = bot.id.to_string();
        let guild_discord_id = guild_id.to_string();

        let list = AutoResponsesQueries::find_by_discord_ids(
            &client.db,
            &bot_discord_id,
            &guild_discord_id
        ).await?
            .iter()
            .map(|autores| autores.trigger.clone())
            .collect::<Vec<String>>();

        if list.len() == 0 {
            let _ = reply_command(
                &client,
                &config,
                &msg,
                "autores-list-empty",
                None,
                ColorResolvables::Yellow
            ).await;
            return Ok(());
        }

        let mut args = FluentArgs::new();
        args.set("count", list.len());

        let mut embed = DiscordEmbed {
            title: Some(
                client.get_locale_string(&config.locale, "autores-list-title", Some(&args))
            ),
            color: Some(ColorResolvables::Blue.as_u32()),
            ..Default::default()
        };

        let mut current_page = 0 as usize;
        let pages = ((list.len() as f32) / (50 as f32)).ceil() as usize;

        fn get_description(list: &Vec<String>, page: usize) -> String {
            list[page * 50..((page + 1) * 50).min(list.len())]
                .to_vec()
                .iter()
                .map(|trigger| format!("`{}`", trigger))
                .collect::<Vec<String>>()
                .join(" ")
                .to_string()
        }

        embed.description = Some(get_description(&list, current_page));

        if pages == 1 {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(vec![embed.clone()])
            ).await;
            return Ok(());
        }

        let unique_key = generate_random_string(10);
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

        let sent_msg = client.http
            .create_message(msg.channel_id)
            .reply(msg.id)
            .embeds(&vec![Embed::from(embed.clone())])?
            .components(
                &vec![
                    Component::ActionRow(ActionRow {
                        components: make_components(&buttons),
                    })
                ]
            )?.await?
            .model().await?;

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
                            embed.description = Some(get_description(&list, current_page));
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
        Ok(())
    }
}
