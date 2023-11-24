pub mod hello_world;

pub mod users_routes;
pub mod bots;
pub mod guilds;
pub mod bot_guild_configs;
pub mod bot_users;
pub mod bot_guild_welcomes;
pub mod bot_logs;
pub mod tickets;

use std::time::{ SystemTime, UNIX_EPOCH };

use crate::{
    database::{
        embed_info::Model as EmbedModel,
        buttons::Model as ButtonModel,
        messages::Model as MessageModel,
    },
    utilities::app_error::AppError,
    queries::message_embed_queries::MessageEmbedQueries,
    default_queries::DefaultSeaQueries,
};
use sea_orm::DatabaseConnection;
use serde::{ Deserialize, Serialize };
use twilight_model::{
    channel::message::{ Embed, embed::{ EmbedFooter, EmbedImage, EmbedThumbnail, EmbedAuthor } },
    util::Timestamp,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    pub id: i32,
    pub r#type: String,
    pub content: Option<String>,
    pub embed_id: Option<i32>,
}

impl From<MessageModel> for ResponseMessage {
    fn from(model: MessageModel) -> Self {
        Self {
            id: model.id,
            r#type: model.r#type,
            content: model.content,
            embed_id: model.embed_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessageDetails {
    pub id: i32,
    pub r#type: String,
    pub content: Option<String>,
    pub embed: Option<ResponseEmbed>,
}

impl ResponseMessage {
    pub async fn to_details(
        &self,
        db: &DatabaseConnection
    ) -> Result<ResponseMessageDetails, AppError> {
        let embed = if let Some(e_id) = self.embed_id {
            let embed_model = MessageEmbedQueries::find_by_id(db, e_id).await?;
            Some(ResponseEmbed::from(embed_model)) // Assuming `From` trait is implemented for `ResponseEmbed`
        } else {
            None
        };

        Ok(ResponseMessageDetails {
            id: self.id,
            r#type: self.r#type.clone(),
            content: self.content.clone(),
            embed,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEmbed {
    pub id: i32,
    pub title: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<i8>,
    pub color: Option<String>,
    pub footer: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub footer_url: Option<String>,
    pub author_url: Option<String>,
}

impl From<EmbedModel> for ResponseEmbed {
    fn from(model: EmbedModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            url: model.url,
            timestamp: model.timestamp,
            color: model.color,
            footer: model.footer,
            image: model.image,
            thumbnail: model.thumbnail,
            author: model.author,
            description: model.description,
            footer_url: model.footer_url,
            author_url: model.author_url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseButton {
    pub id: i32,
    pub color: String,
    pub text: String,
    pub emoji: String,
}

impl From<ButtonModel> for ResponseButton {
    fn from(model: ButtonModel) -> Self {
        Self {
            id: model.id,
            text: model.text,
            color: model.color,
            emoji: model.emoji,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateUpdateMessage {
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<RequestCreateUpdateEmbed>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateUpdateEmbed {
    pub title: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<i8>,
    pub color: Option<String>,
    pub footer: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub footer_url: Option<String>,
    pub author_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateButton {
    pub id: i32,
    pub color: String,
    pub text: String,
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateButton {
    pub id: i32,
    pub color: Option<String>,
    pub text: Option<String>,
    pub emoji: Option<String>,
}

impl From<ResponseEmbed> for Embed {
    fn from(response: ResponseEmbed) -> Self {
        let color = response.color
            .as_ref()
            .and_then(|c| u32::from_str_radix(c.trim_start_matches('#'), 16).ok());

        let timestamp = response.timestamp
            .map(|t| {
                if t != 0 {
                    let since_the_epoch = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    Timestamp::from_micros(since_the_epoch.as_micros() as i64).ok()
                } else {
                    None
                }
            })
            .flatten();

        Embed {
            title: response.title,
            description: response.description,
            url: response.url,
            timestamp,
            color,
            footer: response.footer.map(|text| EmbedFooter {
                text,
                icon_url: response.footer_url,
                proxy_icon_url: None,
            }),
            image: response.image.map(|url| EmbedImage {
                url,
                proxy_url: None,
                height: None,
                width: None,
            }),
            thumbnail: response.thumbnail.map(|url| EmbedThumbnail {
                url,
                proxy_url: None,
                height: None,
                width: None,
            }),
            author: response.author.map(|name| EmbedAuthor {
                name,
                icon_url: response.author_url,
                url: None,
                proxy_icon_url: None,
            }),
            fields: Vec::new(), // Assuming you don't have fields in ResponseEmbed
            kind: "rich".to_string(),
            provider: None,
            video: None,
        }
    }
}
