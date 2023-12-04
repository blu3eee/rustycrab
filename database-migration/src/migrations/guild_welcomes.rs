use rustycrab_api::{
    queries::guild_welcome_queries::GuildWelcomeQueries,
    default_queries::DefaultSeaQueries,
};
use rustycrab_model::{
    error::BoxedError,
    response::{
        bot_guild_welcome::RequestCreateWelcome,
        discord_message::{ RequestCreateUpdateMessage, RequestCreateUpdateEmbed },
    },
};
use sea_orm::{ DatabaseConnection, EntityTrait };
use tokio::task::JoinHandle;

use crate::old_database::{ bot_guild_welcomes as old_wlc, embed_info as old_embed };

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let old_welcomes = old_wlc::Entity::find().all(&old).await?;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for wlc in old_welcomes {
        let old_cloned = old.clone();
        let new_cloned = new.clone();

        let handle = tokio::spawn(async move {
            let message_type = if wlc.embed_info_id.is_some() {
                if wlc.welcome_content.is_some() { "Embed" } else { "Embed & Text" }
            } else {
                "Message"
            };

            let embed: Option<RequestCreateUpdateEmbed> = if
                let Some(old_embed_id) = wlc.embed_info_id
            {
                if
                    let Some(embed_info) = old_embed::Entity
                        ::find_by_id(old_embed_id)
                        .one(&old_cloned).await
                        .ok()
                {
                    if let Some(embed_info) = embed_info {
                        Some(RequestCreateUpdateEmbed {
                            title: embed_info.title,
                            author: embed_info.author,
                            url: embed_info.url,
                            timestamp: embed_info.timestamp,
                            color: embed_info.color,
                            footer: embed_info.footer,
                            image: embed_info.image,
                            thumbnail: embed_info.thumbnail,
                            description: embed_info.description,
                            footer_url: embed_info.footer_icon,
                            author_url: embed_info.author_icon,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            let welcome_message_data = RequestCreateUpdateMessage {
                r#type: Some(message_type.to_string()),
                content: wlc.welcome_content,
                embed,
                ..Default::default()
            };

            let create_req = RequestCreateWelcome {
                bot_discord_id: wlc.bot_bot_id.unwrap(),
                guild_discord_id: wlc.guild_guild_id.unwrap(),
                channel_id: wlc.welcome_channel_id,
                message_data: Some(welcome_message_data),
            };

            match GuildWelcomeQueries::create_entity(&new_cloned, create_req).await {
                Ok(new_wlc) => {
                    println!("created welcome {}", new_wlc.id);
                }
                Err(e) => {
                    eprintln!("error creating wlc: {e:}");
                }
            }
        });

        handles.push(handle);
    }

    // Await all handles to ensure all tasks are completed
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
