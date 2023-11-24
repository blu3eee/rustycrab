mod buttons;

use std::{ error::Error, sync::Arc };

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::{ InteractionType, InteractionData },
};

use crate::twilightrs::{ discord_client::DiscordClient, dispatchers::ClientDispatchers };

use self::buttons::button_handlers;

pub async fn handle_interaction_create(
    client: &Arc<DiscordClient>,
    interaction: &Box<InteractionCreate>,
    dispatchers: &Arc<ClientDispatchers>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(_) = interaction.guild_id {
        match interaction.kind {
            InteractionType::MessageComponent => {
                if let Some(InteractionData::MessageComponent(button_data)) = &interaction.data {
                    button_handlers(client, interaction, dispatchers, button_data).await?;
                }
            }
            InteractionType::ApplicationCommand => {}
            _ => {}
        }
    }
    Ok(())
}
