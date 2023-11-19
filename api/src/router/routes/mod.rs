pub mod hello_world;

pub mod users_routes;
pub mod bots;
pub mod guilds;
pub mod bot_guild_configs;
pub mod bot_users;
pub mod bot_guild_welcomes;
pub mod bot_logs;
pub mod tickets;

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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateUpdateMessage {
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<RequestCreateUpdateEmbed>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateButton {
    pub id: i32,
    pub color: String,
    pub text: String,
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateButton {
    pub id: i32,
    pub color: Option<String>,
    pub text: Option<String>,
    pub emoji: Option<String>,
}
