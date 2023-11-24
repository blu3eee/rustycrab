use std::{ error::Error, sync::Arc, time::{ SystemTime, UNIX_EPOCH }, collections::HashMap };

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    http::{
        permission_overwrite::{ PermissionOverwrite, PermissionOverwriteType },
        attachment::Attachment,
    },
    guild::Guild,
    channel::{
        message::{ MessageFlags, Component, component::{ ActionRow, Button }, Embed },
        Channel,
        ChannelType,
        permission_overwrite::PermissionOverwriteType as ChannelPermissionOverwriteType,
    },
    user::User,
    id::{ Id, marker::{ GuildMarker, UserMarker } },
    guild::{ Permissions, Member },
};

use crate::{
    twilightrs::{ discord_client::DiscordClient, messages::{ DiscordEmbed, DiscordEmbedField } },
    queries::tickets_system::{
        ticket_setting_queries::TicketSettingQueries,
        ticket_queries::TicketQueries,
        ticket_panels_queries::TicketPanelsQueries,
    },
    default_queries::DefaultSeaQueries,
    unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries,
    router::routes::tickets::{
        ticket_settings::ResponseTicketSetting,
        tickets::{
            RequestUpdateTicket,
            ResponseTicketTranscript,
            TranscriptGuild,
            TranscriptChannel,
            TranscriptMessage,
            TranscriptUser,
        },
        ticket_panels::{ ResponseTicketPanelDetails, ResponseTicketPanel },
    },
    database::tickets::Model as TicketModel,
    utilities::utils::color_to_button_style,
    cdn_avatar,
    cdn_guild_icon,
};

pub async fn close_ticket_handler(
    client: &Arc<DiscordClient>,
    interaction: &Box<InteractionCreate>,
    guild_id: Id<GuildMarker>,
    ticket: &TicketModel,
    action: &str
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(status) = &ticket.status {
        if status == "Closed" {
            client.http
                .interaction(interaction.application_id)
                .create_followup(&interaction.token)
                .content(
                    &format!(
                        "<@{}>, this ticket is already closed.",
                        interaction.author().unwrap().id.to_string()
                    )
                )?
                .flags(MessageFlags::EPHEMERAL).await?;
            return Ok(());
        }
    }

    let panel_details = ResponseTicketPanel::from(
        TicketPanelsQueries::find_by_id(&client.db, ticket.panel_id).await?
    ).to_details(&client.db).await?;

    // let bot = client.get_bot().await?;
    let setting: ResponseTicketSetting = TicketSettingQueries::find_by_discord_ids(
        &client.db,
        &panel_details.bot.bot_id,
        &panel_details.guild.guild_id
    ).await?.into();

    let user = interaction.author().unwrap();
    let channel = interaction.channel.as_ref().unwrap();
    let member = client.http
        .guild_member(interaction.guild_id.unwrap(), user.id).await?
        .model().await?;
    if !closing_allowed(client, ticket, &member, channel, &panel_details, &setting).await? {
        client.http
            .interaction(interaction.application_id)
            .create_followup(&interaction.token)
            .content(
                &format!(
                    "{}, you are not allowed to close this ticket.",
                    interaction.author().unwrap().id.to_string()
                )
            )?
            .flags(MessageFlags::EPHEMERAL).await?;
        return Ok(());
    }
    if action == "4" || !setting.ticket_close_confirmation {
        close_confirmed_handler(client, interaction, guild_id, ticket, &setting).await?;
        return Ok(());
    }

    let _ = client.http
        .interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(
            &vec![
                Embed::from(DiscordEmbed {
                    title: Some("Close confirmation".to_string()),
                    description: Some(
                        "Please confirm that you want to close this ticket".to_string()
                    ),
                    author_name: Some(user.name.clone()),
                    author_icon_url: user.avatar.map(|icon| cdn_avatar!(user.id, icon)),
                    ..Default::default()
                })
            ]
        )?
        .components(
            &vec![
                Component::ActionRow(ActionRow {
                    components: vec![
                        Component::Button(Button {
                            custom_id: Some(format!("1:4:{}", ticket.id)),
                            disabled: false,
                            emoji: None,
                            label: Some(format!("Close")),
                            style: color_to_button_style("Blue"),
                            url: None,
                        })
                    ],
                })
            ]
        )?.await;

    Ok(())
}

pub async fn close_confirmed_handler(
    client: &Arc<DiscordClient>,
    interaction: &Box<InteractionCreate>,
    guild_id: Id<GuildMarker>,
    ticket: &TicketModel,
    setting: &ResponseTicketSetting
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = client.http.guild(guild_id).await?.model().await?;
    let ticket_opener = client.http
        .user(Id::new(u64::from_str_radix(&ticket.user_id, 10).unwrap())).await?
        .model().await?;
    let embeds = vec![
        Embed::from(DiscordEmbed {
            author_name: Some(guild.name.clone()),
            author_icon_url: guild.icon.map(|hash| cdn_guild_icon!(guild.id.to_string(), hash)),
            title: Some(format!("Ticket closed")),
            fields: Some(
                vec![
                    DiscordEmbedField {
                        name: "Ticket ID".to_string(),
                        value: format!("{}", ticket.id),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: "Opened by".to_string(),
                        value: format!("<@{}>", ticket.user_id),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: "Closed by".to_string(),
                        value: interaction.author().map_or_else(
                            || format!("[Unknown]"),
                            |author| format!("<@{}>", author.id.to_string())
                        ),
                        inline: true,
                    },
                    DiscordEmbedField {
                        name: "Open time".to_string(),
                        value: format!("<t:{}>", ticket.opened_time),
                        inline: true,
                    }
                ]
            ),
            timestamp: Some(true),
            ..Default::default()
        })
    ];
    let ticket_channel = interaction.channel.as_ref().unwrap();

    // send transcript to the transcript channel
    if let Some(channel_id) = &setting.transcripts_channel {
        if
            let Ok(channel) = client.http.channel(
                Id::new(u64::from_str_radix(channel_id, 10)?)
            ).await
        {
            // create transcript
            let transcript = generate_transcript_data(
                client,
                &guild,
                &ticket_channel,
                &ticket_opener
            ).await?;

            let json = serde_json::to_vec(&transcript).expect("Serialization failed");
            let mut attachment = Attachment::from_bytes(
                format!("ticket_transcript_{}.json", ticket.id),
                json,
                1
            );
            attachment.description = Some(format!("transcript for ticket{}", ticket.id));

            // send message with link button
            let transcript_channel = channel.model().await?;
            let transcript_message = client.http
                .create_message(transcript_channel.id)
                .attachments(&vec![attachment])?
                .embeds(&embeds)?.await?
                .model().await?;
            let _ = TicketQueries::update_by_id(&client.db, ticket.id, RequestUpdateTicket {
                transcript_message_id: Some(transcript_message.id.to_string()),
                transcript_channel_id: Some(transcript_message.channel_id.to_string()),
                ..Default::default()
            }).await;
        }
    }

    if let Some(channel) = &interaction.channel {
        // send closed ticket message in channel

        // close ticket thread or change permission if ticekt is not thread
        if channel.kind == ChannelType::GuildText {
            // deny all the channel previous permission overwrites
            let ticket_channel = client.http.channel(ticket_channel.id).await?.model().await?;
            // println!("permission overwrites {:?}", &ticket_channel.permission_overwrites);

            if let Some(overwrites) = &ticket_channel.permission_overwrites {
                for overwrite in overwrites {
                    // println!("denying permission {:?}", overwrite);
                    let _ = client.http.update_channel_permission(
                        channel.id,
                        &(PermissionOverwrite {
                            kind: if overwrite.kind == ChannelPermissionOverwriteType::Member {
                                PermissionOverwriteType::Member
                            } else {
                                PermissionOverwriteType::Role
                            },
                            id: overwrite.id,
                            allow: None,
                            deny: Some(overwrite.allow),
                        })
                    ).await;
                }
            }
            // First, try to get the main archive category
            let main_category = get_category_channel(client, &setting.archive_category).await?;

            // Then, try to get the overflow archive category if the main one is not available
            let overflow_category = if main_category.is_none() {
                get_category_channel(client, &setting.archive_overflow_category).await?
            } else {
                None
            };

            let category = main_category.or(overflow_category);

            let _ = client.http
                .update_channel(channel.id)
                .name(&format!("closed-{}", channel.name.clone().unwrap()))?
                .parent_id(category.map(|category| category.id)).await;
        } else {
            client.http
                .update_thread(channel.id)
                .name(&format!("closed-{}", channel.name.clone().unwrap()))?
                .locked(true)
                .archived(true).await?;
        }
    }
    // send closing notification to the ticket opener
    if let Ok(dm_channel) = client.http.create_private_channel(ticket_opener.id).await {
        if let Ok(dm_channel) = dm_channel.model().await {
            let _ = client.http.create_message(dm_channel.id).embeds(&embeds)?.await;
        }
    }

    // send closed notification to the ticket channel
    // let _ = client.http.create_message(ticket_channel.id).embeds(&embeds)?.await;
    let _ = client.http
        .interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(&embeds)?.await;

    // update ticket status
    let _ = TicketQueries::update_by_id(&client.db, ticket.id, RequestUpdateTicket {
        status: Some("Closed".to_string()),
        ..Default::default()
    }).await;

    Ok(())
}

async fn closing_allowed(
    client: &Arc<DiscordClient>,
    ticket: &TicketModel,
    member: &Member,
    channel: &Channel,
    panel_details: &ResponseTicketPanelDetails,
    setting: &ResponseTicketSetting
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if setting.allow_user_to_close_tickets && member.user.id.to_string() == ticket.user_id {
        return Ok(true);
    }
    let user_permissions = client.cache.permissions().in_channel(member.user.id, channel.id)?;

    if user_permissions.contains(Permissions::ADMINISTRATOR) {
        return Ok(true);
    }

    if let Some(team) = &panel_details.support_team {
        if team.users.contains(&member.user.id.to_string()) {
            return Ok(true);
        }

        let role_ids = member.roles
            .clone()
            .into_iter()
            .map(|role| role.to_string())
            .collect::<Vec<String>>();

        if
            team.roles
                .clone()
                .into_iter()
                .any(|role_id| role_ids.contains(&role_id))
        {
            return Ok(true);
        }
    }

    return Ok(false);
}

async fn generate_transcript_data(
    client: &Arc<DiscordClient>,
    guild: &Guild,
    channel: &Channel,
    ticket_opener: &User
) -> Result<ResponseTicketTranscript, Box<dyn Error + Send + Sync>> {
    let messages = client.fetch_messages(channel).await?;

    let mut users: HashMap<Id<UserMarker>, User> = HashMap::new();
    let transcript_messages = messages
        .into_iter()
        .map(|message| {
            users.entry(message.author.id).or_insert_with(|| message.author.clone());
            TranscriptMessage {
                user_id: message.id.to_string(),
                content: message.content,
                embeds: vec![],
                attachments: vec![],
                timestamp: message.timestamp.as_secs() as i32,
            }
        })
        .collect::<Vec<TranscriptMessage>>();

    let transcript_users = users
        .values()
        .map(|user| TranscriptUser {
            id: user.id.to_string(),
            name: user.name.clone(),
            avatar_url: user.avatar.map_or_else(
                || format!(""),
                |hash| format!("{}", hash)
            ),
            bot: user.bot,
        })
        .collect::<Vec<TranscriptUser>>();

    Ok(ResponseTicketTranscript {
        generated: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i32,
        guild: TranscriptGuild {
            guild_id: guild.id.to_string(),
            name: guild.name.clone(),
            icon_url: guild.icon.map_or_else(
                || format!(""),
                |hash| hash.to_string()
            ),
        },
        channel: TranscriptChannel {
            id: channel.id.to_string(),
            name: channel.name.as_ref().map_or_else(
                || format!("ticket"),
                |name| format!("{}", name)
            ),
        },
        ticket_opener_id: ticket_opener.id.to_string(),
        users: transcript_users,
        messages: transcript_messages,
    })
}

async fn get_category_channel(
    client: &Arc<DiscordClient>,
    category_id_option: &Option<String>
) -> Result<Option<Channel>, Box<dyn Error + Send + Sync>> {
    if let Some(category_id_str) = category_id_option {
        if let Ok(category_id) = u64::from_str_radix(category_id_str, 10) {
            if let Ok(category) = client.http.channel(Id::new(category_id)).await {
                return Ok(Some(category.model().await?));
            }
        }
    }
    Ok(None)
}
