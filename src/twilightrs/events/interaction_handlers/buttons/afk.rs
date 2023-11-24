use std::{ sync::Arc, error::Error };
use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::{ Embed, MessageFlags },
    id::Id,
};

use crate::{
    twilightrs::{ discord_client::DiscordClient, messages::DiscordEmbed },
    utilities::utils::ColorResolvables,
};

pub async fn add_afk_notification(
    client: &Arc<DiscordClient>,
    interaction: &Box<InteractionCreate>,
    button_data: &MessageComponentInteractionData
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("acked {}", button_data.custom_id);
    client.defer_button_interaction(interaction).await?;

    if let Some(user) = interaction.author() {
        let afk_user_id = button_data.custom_id
            .split(':')
            .nth(1)
            .and_then(|id| id.parse::<u64>().ok());

        let added = if let Some(afk_user_id) = afk_user_id {
            let mut afk_users = client.afk_users.write().unwrap();
            let notify_user_id = user;
            let guild_id = interaction.guild_id.unwrap(); // Assuming this is always in a guild context

            if
                let Some(afk_user) = afk_users
                    .get_mut(&guild_id)
                    .and_then(|g| g.get_mut(&Id::new(afk_user_id)))
            {
                if !afk_user.notify.contains(&notify_user_id.id) {
                    afk_user.notify.push(notify_user_id.id);
                }
                true
            } else {
                false
            }
        } else {
            false
        };

        let locale = if let Some(guild_id) = interaction.guild_id {
            if let Ok(config) = client.get_guild_config(&guild_id).await {
                config.locale
            } else {
                "en".to_string()
            }
        } else {
            "en".to_string()
        };

        let (content, color) = if added {
            (client.get_locale_string(&locale, "afk-notify-added", None), ColorResolvables::Green)
        } else {
            (client.get_locale_string(&locale, "afk-notfound", None), ColorResolvables::Yellow)
        };

        let _ = client.http
            .interaction(interaction.application_id)
            .create_followup(&interaction.token)
            .embeds(
                &vec![
                    Embed::from(DiscordEmbed {
                        description: Some(content),
                        color: Some(color.as_u32()),
                        ..Default::default()
                    })
                ]
            )?
            .flags(MessageFlags::EPHEMERAL).await;
    }
    Ok(())
}
