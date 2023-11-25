use std::{ error::Error, sync::Arc };

use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    gateway::payload::incoming::InteractionCreate,
};

use crate::twilightrs::{ dispatchers::ClientDispatchers, discord_client::DiscordClient };

use self::{ tickets::tickets_handler, afk::add_afk_notification };

mod tickets;
mod afk;

pub async fn button_handlers(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    dispatchers: &Arc<ClientDispatchers>,
    button_data: &MessageComponentInteractionData
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let button_parts: Vec<String> = button_data.custom_id.split(":").map(String::from).collect();
    if let Some(button_event) = button_parts.first() {
        match button_event.as_str() {
            "1" => {
                tickets_handler(client, interaction, dispatchers, button_data).await?;
            }
            "2" => {
                add_afk_notification(client, interaction, button_data).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
