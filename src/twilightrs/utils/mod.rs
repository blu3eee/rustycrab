use std::error::Error;

use fluent_bundle::FluentArgs;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{ utilities::utils::ColorResolvables, cdn_avatar };

use super::{
    discord_client::{ DiscordClient, MessageContent },
    commands::context::context_command::GuildConfigModel,
    messages::DiscordEmbed,
};

pub mod greedy;
pub mod discord_embed_builder;
pub mod afk;

pub async fn send_response_message(
    client: &DiscordClient,
    config: &GuildConfigModel,
    msg: &MessageCreate,
    key: &str,
    color: ColorResolvables
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let content = client.get_locale_string(&config.locale, key, None);
    client.reply_message(
        msg.channel_id,
        msg.id,
        MessageContent::DiscordEmbeds(
            vec![DiscordEmbed {
                description: Some(content),
                color: Some(color.as_u32()),
                footer_text: Some(
                    client.get_locale_string(
                        &config.locale,
                        "requested-user",
                        Some(&FluentArgs::from_iter(vec![("username", msg.author.name.clone())]))
                    )
                ),
                footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                ..Default::default()
            }]
        )
    ).await?;

    Ok(())
}
