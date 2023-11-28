use std::{ error::Error, sync::Arc, str::FromStr };
use enum_primitive_derive::Primitive;
use num_traits::{ ToPrimitive, FromPrimitive };
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    gateway::payload::incoming::InteractionCreate,
};

use crate::twilightrs::{ dispatchers::ClientDispatchers, discord_client::DiscordClient };

use self::{ tickets::tickets_handler, afk::add_afk_notification };

mod tickets;
mod afk;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum ButtonEvents {
    Tickets = 1,
    Afk = 2,
    MusicPlayer = 3,
}

impl FromStr for ButtonEvents {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(num) => ButtonEvents::from_i32(num).ok_or(()),
            Err(_) => Err(()),
        }
    }
}

impl ButtonEvents {
    #[allow(dead_code)]
    pub fn to_i32_string(&self) -> String {
        self.to_i32().unwrap().to_string()
    }
}

pub async fn button_handlers(
    client: DiscordClient,
    interaction: &Box<InteractionCreate>,
    dispatchers: &Arc<ClientDispatchers>,
    button_data: &MessageComponentInteractionData
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let button_parts: Vec<String> = button_data.custom_id.split(":").map(String::from).collect();
    if let Some(button_event) = button_parts.first() {
        if let Ok(button_event) = ButtonEvents::from_str(button_event.as_str()) {
            match button_event {
                ButtonEvents::Tickets => {
                    tickets_handler(client, interaction, dispatchers, button_data).await?;
                }
                ButtonEvents::Afk => {
                    add_afk_notification(client, interaction, button_data).await?;
                }
                ButtonEvents::MusicPlayer => {
                    // music_player_handler(client, interaction, button_data).await?;
                }
                // _ => {}
            }
        }
    }
    Ok(())
}
