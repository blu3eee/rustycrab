pub mod buttons;

use std::{ error::Error, sync::Arc };

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::{ InteractionType, InteractionData },
};

use crate::twilightrs::{ discord_client::DiscordClient, dispatchers::ClientDispatchers };

use self::buttons::button_handlers;

pub async fn handle_interaction_create(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    dispatchers: &Arc<ClientDispatchers>
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if let Some(_) = interaction.guild_id {
        match interaction.kind {
            InteractionType::MessageComponent => {
                if let Some(InteractionData::MessageComponent(button_data)) = &interaction.data {
                    button_handlers(client, interaction, dispatchers, button_data).await?;
                }
            }
            InteractionType::ApplicationCommand => {
                if let Some(InteractionData::ApplicationCommand(command_data)) = &interaction.data {
                    // command_data.name
                    let _ = dispatchers.slash_commands.dispatch(
                        client,
                        interaction,
                        command_data
                    ).await;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
