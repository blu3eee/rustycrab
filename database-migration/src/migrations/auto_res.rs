use rustycrab_api::{
    queries::auto_responses_queries::AutoResponsesQueries,
    default_queries::DefaultSeaQueries,
};
use rustycrab_model::{
    response::{
        auto_response::RequestCreateAutoResponse,
        discord_message::{ RequestCreateUpdateMessage, RequestCreateUpdateEmbed },
    },
    error::BoxedError,
};
use sea_orm::{ EntityTrait, DatabaseConnection };
use tokio::task::JoinHandle;

use crate::old_database::{ auto_responses as old_ar, embed_info as old_embed };

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let old_ars = old_ar::Entity::find().all(&old).await?;

    println!("found {} old autoresponses", old_ars.len());

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for ar in old_ars {
        let old_cloned = old.clone();
        let new_cloned = new.clone();

        let handle = tokio::spawn(async move {
            let ar_type = if ar.is_embed != 0 {
                if ar.response_list.is_empty() { "Embed" } else { "Embed & Text" }
            } else {
                "Message"
            };
            let embed: Option<RequestCreateUpdateEmbed> = if
                let Some(old_embed_id) = ar.embed_info_id
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

            let create_req = RequestCreateAutoResponse {
                bot_discord_id: ar.bot_id,
                guild_discord_id: ar.guild_id,
                trigger: ar.trigger,
                response_data: RequestCreateUpdateMessage {
                    r#type: Some(ar_type.to_string()),
                    content: Some(ar.response_list),
                    embed: if ar.is_embed != 0 {
                        embed
                    } else {
                        None
                    },
                    ..Default::default()
                },
            };

            match AutoResponsesQueries::create_entity(&new_cloned, create_req).await {
                Ok(new_autores) => { println!("created autores {}", new_autores.id) }
                Err(e) => { eprintln!("error creating autores {e:?}") }
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
