use std::{ error::Error, sync::Arc };

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::MessageFlags,
};

use crate::{
    twilightrs::{ discord_client::DiscordClient, dispatchers::ClientDispatchers },
    queries::tickets_system::{
        ticket_panels_queries::TicketPanelsQueries,
        ticket_queries::TicketQueries,
    },
    default_queries::DefaultSeaQueries,
    router::routes::tickets::ticket_panels::{ ResponseTicketPanel, ResponseTicketPanelDetails },
};

use self::{ open_ticket::open_ticket_handler, close_ticket::close_ticket_handler };

mod open_ticket;
mod close_ticket;

pub async fn tickets_handler(
    client: &Arc<DiscordClient>,
    interaction: &Box<InteractionCreate>,
    _: &Arc<ClientDispatchers>,
    button_data: &MessageComponentInteractionData
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(guild_id) = interaction.guild_id {
        client.defer_interaction(interaction).await?;

        let button_parts: Vec<String> = button_data.custom_id
            .split(":")
            .map(String::from)
            .collect();

        if let Some(action) = button_parts.get(1) {
            match action.as_str() {
                "1" => {
                    if let Some(id) = button_parts.get(2) {
                        let panel_id = i32::from_str_radix(id, 10)?;
                        let panel: ResponseTicketPanel = TicketPanelsQueries::find_by_id(
                            &client.db,
                            panel_id
                        ).await?.into();

                        let panel_details: ResponseTicketPanelDetails = panel.to_details(
                            &client.db
                        ).await?;
                        open_ticket_handler(client, interaction, guild_id, panel_details).await?;
                    } else {
                        client.http
                            .interaction(interaction.application_id)
                            .create_followup(&interaction.token)
                            .content("Unknown interaction")?
                            .flags(MessageFlags::EPHEMERAL).await?;
                    }
                }
                "3" | "4" => {
                    if let Some(id) = button_parts.get(2) {
                        let ticket_id = i32::from_str_radix(id, 10)?;
                        let ticket = TicketQueries::find_by_id(&client.db, ticket_id).await;
                        if let Ok(ticket) = ticket {
                            close_ticket_handler(
                                client,
                                interaction,
                                guild_id,
                                &ticket,
                                &action
                            ).await?;
                        } else {
                            client.http
                                .interaction(interaction.application_id)
                                .create_followup(&interaction.token)
                                .content("Unknown ticket")?
                                .flags(MessageFlags::EPHEMERAL).await?;
                        }
                    } else {
                        client.http
                            .interaction(interaction.application_id)
                            .create_followup(&interaction.token)
                            .content("Unknown interaction")?
                            .flags(MessageFlags::EPHEMERAL).await?;
                    }
                }
                _ => {
                    client.http
                        .interaction(interaction.application_id)
                        .create_followup(&interaction.token)
                        .content("Unknown ticket action")?
                        .flags(MessageFlags::EPHEMERAL).await?;
                }
            }
        }
    }
    Ok(())
}
