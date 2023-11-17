pub mod hello_world;
pub mod bots;
pub mod users_routes;
pub mod guilds;

pub mod bot_guild_configs;
pub mod discord_client;
pub mod bot_users;
pub mod guild_welcomes;
pub mod bot_logs;

use crate::database::{ embed_info::Model as EmbedModel, buttons::Model as ButtonModel };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub id: i32,
    pub r#type: String,
    pub content: Option<String>,
    pub embed: Option<ResponseEmbed>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Deserialize)]
pub struct RequestCreateUpdateMessage {
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<RequestCreateUpdateEmbed>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct RequestCreateButton {
    pub id: i32,
    pub color: String,
    pub text: String,
    pub emoji: String,
}

#[derive(Deserialize)]
pub struct RequestUpdateButton {
    pub id: i32,
    pub color: Option<String>,
    pub text: Option<String>,
    pub emoji: Option<String>,
}
