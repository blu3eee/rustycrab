use std::{ error::Error, sync::Arc, time::{ SystemTime, UNIX_EPOCH } };

use rustycrab_model::response::ticket::{
    panel::ResponseTicketPanelDetails,
    setting::ResponseTicketSetting,
    ticket::{ RequestCreateTicket, RequestUpdateTicket },
};
use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    http::permission_overwrite::{ PermissionOverwrite, PermissionOverwriteType },
    channel::{
        message::{ MessageFlags, Component, component::{ ActionRow, Button }, Embed },
        ChannelType,
    },
    user::User,
    id::{ Id, marker::{ GuildMarker, ChannelMarker } },
    guild::Permissions,
};

use crate::{
    twilightrs::{ discord_client::DiscordClient, messages::DiscordEmbed },
    queries::{
        tickets_system::{
            ticket_setting_queries::TicketSettingQueries,
            ticket_queries::TicketQueries,
            ticket_panels_queries::TicketPanelsQueries,
        },
        message_embed_queries::MessageEmbedQueries,
    },
    default_queries::DefaultSeaQueries,
    unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries,
    database::tickets::Model as TicketModel,
    utilities::utils::color_to_button_style,
};

pub async fn open_ticket_handler(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    guild_id: Id<GuildMarker>,
    panel_id: String
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if let Some(user) = interaction.author() {
        let panel_id = i32::from_str_radix(&panel_id, 10)?;

        let panel_details: ResponseTicketPanelDetails = TicketPanelsQueries::fetch_panel_details(
            &client.db,
            panel_id
        ).await?;

        let setting: ResponseTicketSetting = TicketSettingQueries::find_by_discord_ids(
            &client.db,
            &panel_details.bot.bot_id,
            &panel_details.guild.guild_id
        ).await?.into();

        let current_tickets = TicketQueries::find_user_tickets(
            &client.db,
            user.id.to_string()
        ).await?;
        if (current_tickets.len() as i32) >= setting.per_user_ticket_limit {
            client.http
                .interaction(interaction.application_id)
                .create_followup(&interaction.token)
                .content("You have reached ticket limit")?
                .flags(MessageFlags::EPHEMERAL).await?;
            return Ok(());
        }

        let ticket = TicketQueries::create_entity(&client.db, RequestCreateTicket {
            bot_discord_id: panel_details.bot.bot_id.clone(),
            guild_discord_id: panel_details.guild.guild_id.clone(),
            user_id: user.id.to_string(),
            panel_id: panel_details.id,
            opened_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i32,
        }).await?;
        // Check if the setting is for thread or channel ticket
        let channel = if setting.thread_ticket {
            create_thread_ticket(
                Arc::clone(&client),
                interaction,
                &panel_details,
                user,
                guild_id,
                &ticket
            ).await?
        } else {
            create_channel_ticket(
                Arc::clone(&client),
                interaction,
                &panel_details,
                user,
                guild_id,
                &ticket
            ).await?
        };

        if let Some(channel_id) = channel {
            welcome_ticket(
                Arc::clone(&client),
                interaction,
                panel_details,
                user,
                guild_id,
                ticket,
                channel_id
            ).await?;
        } else {
            client.http
                .interaction(interaction.application_id)
                .create_followup(&interaction.token)
                .content("Failed to create ticket for you")?
                .flags(MessageFlags::EPHEMERAL).await?;
        }
    } else {
        println!("cant find interacted user");
    }
    Ok(())
}

async fn create_thread_ticket(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    panel_details: &ResponseTicketPanelDetails,
    user: &User,
    _: Id<GuildMarker>,
    ticket: &TicketModel
) -> Result<Option<Id<ChannelMarker>>, Box<dyn Error + Send + Sync>> {
    // Implementation to create a thread and set permissions
    // Ensure to use appropriate methods from Discord API client to create threads
    // Define the channel where the thread will be created (this should be set in your panel settings)
    if let Some(channel) = &interaction.channel {
        let parent_channel_id: Id<ChannelMarker> = channel.id;

        // Create a thread in the specified channel
        let thread_name = format!(
            "{}",
            panel_details.naming_scheme
                .replace("{id}", &ticket.id.to_string())
                .replace("{username}", &user.name)
        ); // Customizable thread name
        let thread = client.http.create_thread(
            parent_channel_id,
            &thread_name,
            ChannelType::PrivateThread
        )?.await;

        let thread_id = match thread {
            Ok(thread) => { thread.model().await?.id }
            Err(_) => {
                return Err("Failed to create thread".into());
            }
        };

        return Ok(Some(thread_id));
    }
    return Ok(None);
}

async fn create_channel_ticket(
    client: DiscordClient,
    _: &Box<InteractionCreate>,
    panel_details: &ResponseTicketPanelDetails,
    user: &User,
    guild_id: Id<GuildMarker>,
    ticket: &TicketModel
) -> Result<Option<Id<ChannelMarker>>, Box<dyn Error + Send + Sync>> {
    // Implementation to create a channel and set permissions
    // Ensure to use appropriate methods from Discord API client to create channels and manage permissions
    let ticket_category = if panel_details.ticket_category.is_empty() {
        None
    } else {
        if
            let Ok(category) = client.http.channel(
                Id::new(u64::from_str_radix(&panel_details.ticket_category, 10).unwrap())
            ).await
        {
            Some(category.model().await?.id)
        } else {
            None
        }
    };

    let ticket_name = panel_details.naming_scheme
        .replace("{id}", &ticket.id.to_string())
        .replace("{username}", &user.name);
    let ticket_channel = client.http.create_guild_channel(guild_id, &ticket_name)?;

    let ticket_channel = if let Some(category_id) = ticket_category {
        ticket_channel.parent_id(category_id).await
    } else {
        ticket_channel.await
    };

    let ticket_channel_id = match ticket_channel {
        Ok(channel) => { channel.model().await?.id }
        Err(_) => {
            return Err("Failed to create channel".into());
        }
    };

    // Set permissions for the user who initiated the ticket
    let ticket_opener_permission_overwrites =
        Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY;
    let _ = client.http.update_channel_permission(
        ticket_channel_id,
        &(PermissionOverwrite {
            kind: PermissionOverwriteType::Member,
            id: Id::new(user.id.into()),
            allow: Some(ticket_opener_permission_overwrites),
            deny: None,
        })
    ).await;

    if let Some(team) = &panel_details.support_team {
        let support_team_permission_overwrites =
            Permissions::VIEW_CHANNEL |
            Permissions::SEND_MESSAGES |
            Permissions::READ_MESSAGE_HISTORY |
            Permissions::MANAGE_MESSAGES |
            Permissions::MANAGE_ROLES;
        for role_id in &team.roles {
            if !role_id.is_empty() {
                let _ = client.http.update_channel_permission(
                    ticket_channel_id,
                    &(PermissionOverwrite {
                        kind: PermissionOverwriteType::Role,
                        id: Id::new(u64::from_str_radix(role_id, 10)?.into()),
                        allow: Some(support_team_permission_overwrites),
                        deny: None,
                    })
                ).await;
            }
        }
        for user_id in &team.users {
            if !user_id.is_empty() {
                let _ = client.http.update_channel_permission(
                    ticket_channel_id,
                    &(PermissionOverwrite {
                        kind: PermissionOverwriteType::Member,
                        id: Id::new(u64::from_str_radix(user_id, 10)?.into()),
                        allow: Some(support_team_permission_overwrites),
                        deny: None,
                    })
                ).await;
            }
        }
    }

    Ok(Some(ticket_channel_id))
}

async fn welcome_ticket(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    panel_details: ResponseTicketPanelDetails,
    user: &User,
    guild_id: Id<GuildMarker>,
    ticket: TicketModel,
    channel_id: Id<ChannelMarker>
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // mention suppor team

    if let Some(team) = &panel_details.support_team {
        let notify_support_team_msg = client.send_message(
            channel_id,
            crate::twilightrs::discord_client::MessageContent::Text(
                format!(
                    "[Support team] {}, {}",
                    &team.roles
                        .clone()
                        .into_iter()
                        .map(|role_id| format!("<@&{}>", role_id))
                        .collect::<Vec<String>>()
                        .join(" "),
                    &team.users
                        .clone()
                        .into_iter()
                        .map(|user_id| format!("<@{}>", user_id))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            )
        ).await;
        if let Ok(msg) = notify_support_team_msg {
            let msg = msg.model().await;
            if let Ok(msg) = msg {
                let _ = client.http.delete_message(channel_id, msg.id).await;
            }
        }
    }

    //  send welcome message
    if let Some(message) = &panel_details.welcome_message {
        if let Some(embed) = &message.embed {
            println!("building embeds");
            let embeds = vec![
                DiscordEmbed::from(
                    MessageEmbedQueries::find_by_id(&client.db, embed.id).await?
                ).to_embed(&client.http, Some(guild_id), Some(user.id)).await
            ];
            let components = vec![
                Component::ActionRow(ActionRow {
                    components: vec![
                        Component::Button(Button {
                            custom_id: Some(format!("1:3:{}", ticket.id)),
                            disabled: false,
                            emoji: None,
                            label: Some(format!("Close")),
                            style: color_to_button_style("Red"),
                            url: None,
                        })
                    ],
                })
            ];

            let msg = client.http
                .create_message(channel_id)
                .embeds(&embeds)?
                .components(&components)?.await;

            if let Err(err) = msg {
                eprintln!("Error sending welcome message {}", err);
            }
        }
    }

    // mention on open
    let mentions: String = panel_details.mention_on_open
        .clone()
        .into_iter()
        .map(|mention| {
            if mention.eq("ticket-opener") {
                format!("<@{}>", user.id.to_string())
            } else {
                format!("<@&{}>", mention)
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    let _ = client.send_message(
        channel_id,
        crate::twilightrs::discord_client::MessageContent::Text(mentions)
    ).await;

    // notify the user that their ticket channel is opened
    let _ = client.http
        .interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(
            &vec![
                Embed::from(DiscordEmbed {
                    description: Some(format!("Ticket created at <#{}>", channel_id.to_string())),
                    ..Default::default()
                })
            ]
        )?
        .flags(MessageFlags::EPHEMERAL).await;

    let _ = TicketQueries::update_by_id(&client.db, ticket.id, RequestUpdateTicket {
        channel_id: Some(channel_id.to_string()),
        status: Some("Opened".to_string()),
        ..Default::default()
    }).await;

    Ok(())
}
