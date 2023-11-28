use std::error::Error;

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::application_command::CommandData,
    http::interaction::{ InteractionResponseType, InteractionResponse, InteractionResponseData },
    channel::message::MessageFlags,
};

use async_trait::async_trait;

use crate::{
    twilightrs::{
        commands::slash::slash_command::SlashCommand,
        discord_client::DiscordClient,
        bot::tickets::close_ticket_handler,
    },
    queries::tickets_system::ticket_queries::TicketQueries,
};

pub struct CloseTicketSlashCommand {}

#[async_trait]
impl SlashCommand for CloseTicketSlashCommand {
    fn name(&self) -> &'static str {
        "closeticket"
    }

    fn description(&self) -> &'static str {
        "close the current ticket"
    }

    async fn run(
        &self,
        client: DiscordClient,
        interaction: &Box<InteractionCreate>,
        _: &Box<CommandData>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(guild_id) = &interaction.guild_id {
            if let Some(channel) = &interaction.channel {
                println!("close ticket");

                if
                    let Ok(ticket) = TicketQueries::find_by_channel_discord_id(
                        &client.db,
                        channel.id.to_string()
                    ).await
                {
                    client.http.interaction(interaction.application_id).create_response(
                        interaction.id,
                        &interaction.token,
                        &(InteractionResponse {
                            kind: InteractionResponseType::DeferredChannelMessageWithSource,
                            data: None,
                        })
                    ).await?;

                    let _ = close_ticket_handler(client, interaction, guild_id, &ticket, "3").await;
                } else {
                    client.http.interaction(interaction.application_id).create_response(
                        interaction.id,
                        &interaction.token,
                        &(InteractionResponse {
                            kind: InteractionResponseType::DeferredChannelMessageWithSource,
                            data: Some(InteractionResponseData {
                                flags: Some(MessageFlags::EPHEMERAL),
                                ..Default::default()
                            }),
                        })
                    ).await?;

                    let _ = client.http
                        .interaction(interaction.application_id)
                        .create_followup(&interaction.token)
                        .content("This is not a ticket")?.await;
                }
            }
        }

        Ok(())
    }
}
