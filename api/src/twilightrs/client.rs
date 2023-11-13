use std::time::{ SystemTime, UNIX_EPOCH };

use sea_orm::DatabaseConnection;
use twilight_model::{
    channel::{
        Message,
        message::embed::{ Embed, EmbedAuthor, EmbedFooter, EmbedImage, EmbedThumbnail, EmbedField },
    },
    id::{ Id, marker::{ ChannelMarker, MessageMarker } },
    util::Timestamp,
};
use twilight_http::{ Client as HttpClient, Response, request::channel::message::CreateMessage };
use std::{ sync::Arc, error::Error };

use crate::database::embed_info::Model as EmbedModel;

#[derive(Debug, Clone, Default)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u64>,
    pub footer_text: Option<String>,
    pub footer_icon_url: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub author_name: Option<String>,
    pub author_icon_url: Option<String>,
    pub fields: Option<Vec<DiscordEmbedField>>,
}

#[derive(Debug, Clone)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
pub enum MessageContent {
    Text(String),
    EmbedModels(Vec<EmbedModel>),
    TextAndEmbedModels(String, Vec<EmbedModel>),
    DiscordEmbeds(Vec<DiscordEmbed>),
    TextAndDiscordEmbeds(String, Vec<DiscordEmbed>),
    None,
}

#[derive(Clone)]
pub struct DiscordClient {
    pub db: DatabaseConnection,
    pub http: Arc<HttpClient>,
}

impl DiscordClient {
    fn convert_embed_models(&self, embed_models: Vec<EmbedModel>) -> Vec<Embed> {
        embed_models.into_iter().map(Embed::from).collect()
    }

    fn convert_discord_embeds(&self, discord_embeds: Vec<DiscordEmbed>) -> Vec<Embed> {
        discord_embeds.into_iter().map(Embed::from).collect()
    }

    async fn send_discord_message(
        &self,
        create_message: CreateMessage<'_>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        match message_content {
            MessageContent::Text(text) => { Ok(create_message.content(&text)?.await?) }
            MessageContent::EmbedModels(embeds) => {
                Ok(create_message.embeds(&self.convert_embed_models(embeds))?.await?)
            }
            MessageContent::TextAndEmbedModels(text, embeds) => {
                Ok(
                    create_message
                        .content(&text)?
                        .embeds(&self.convert_embed_models(embeds))?.await?
                )
            }
            MessageContent::DiscordEmbeds(embeds) => {
                Ok(create_message.embeds(&self.convert_discord_embeds(embeds))?.await?)
            }
            MessageContent::TextAndDiscordEmbeds(text, embeds) => {
                Ok(
                    create_message
                        .content(&text)?
                        .embeds(&self.convert_discord_embeds(embeds))?.await?
                )
            }
            MessageContent::None => {
                // Handle case where no content is provided (might do nothing or give an error)
                Err("No content provided for reply".into())
            }
        }
    }

    pub async fn send_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        self.send_discord_message(self.http.create_message(channel_id), message_content).await
    }

    pub async fn reply_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        self.send_discord_message(
            self.http.create_message(channel_id).reply(message_id),
            message_content
        ).await
    }

    pub async fn edit_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        let message_update = self.http.update_message(channel_id, message_id);

        match message_content {
            MessageContent::Text(text) => { Ok(message_update.content(Some(&text))?.await?) }
            MessageContent::EmbedModels(embeds) => {
                Ok(message_update.embeds(Some(&self.convert_embed_models(embeds)))?.await?)
            }
            MessageContent::TextAndEmbedModels(text, embeds) => {
                Ok(
                    message_update
                        .content(Some(&text))?
                        .embeds(Some(&self.convert_embed_models(embeds)))?.await?
                )
            }
            MessageContent::DiscordEmbeds(embeds) => {
                Ok(message_update.embeds(Some(&self.convert_discord_embeds(embeds)))?.await?)
            }
            MessageContent::TextAndDiscordEmbeds(text, embeds) => {
                Ok(
                    message_update
                        .content(Some(&text))?
                        .embeds(Some(&self.convert_discord_embeds(embeds)))?.await?
                )
            }
            MessageContent::None => {
                // Handle case where no content is provided (might clear the content or do nothing)
                Err("No content provided for edit".into())
            }
        }
    }
}

impl From<EmbedModel> for Embed {
    fn from(model: EmbedModel) -> Self {
        Embed {
            author: model.author.map(|name| EmbedAuthor {
                name,
                icon_url: model.author_url,
                proxy_icon_url: None,
                url: None, // If you have a corresponding URL in EmbedModel, use it here
            }),
            color: u32
                ::from_str_radix(
                    &model.color.unwrap_or_else(|| "2B2D31".to_string()).trim_start_matches('#'),
                    16
                )
                .ok(),
            description: model.description,
            fields: vec![], // Assuming you have a way to convert model fields to Vec<EmbedField>
            footer: model.footer.map(|text| EmbedFooter {
                text,
                icon_url: model.footer_url,
                proxy_icon_url: None, // Proxy URLs are generally provided by Discord, not by the user
            }),
            image: model.image.map(|url| EmbedImage {
                url,
                proxy_url: None, // Proxy URLs are generally provided by Discord, not by the user
                height: None, // You may not have this information
                width: None, // You may not have this information
            }),
            kind: "rich".to_string(), // "rich" is a commonly used kind
            provider: None, // This is generally set by Discord when using links from known providers
            thumbnail: model.thumbnail.map(|url| EmbedThumbnail {
                url,
                proxy_url: None,
                height: None,
                width: None,
            }),
            timestamp: model.timestamp
                .map(|t| {
                    if t == 0 {
                        None
                    } else {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        Timestamp::from_micros(since_the_epoch.as_micros() as i64).ok()
                    }
                })
                .flatten(),
            title: model.title,
            url: model.url,
            video: None, // Assuming you don't have video information in your model
        }
    }
}

impl From<DiscordEmbed> for Embed {
    fn from(model: DiscordEmbed) -> Self {
        Embed {
            author: model.author_name.map(|name| EmbedAuthor {
                name,
                icon_url: model.author_icon_url,
                proxy_icon_url: None,
                url: None, // If you have a corresponding URL in EmbedModel, use it here
            }),
            // color: model.color.and_then(|c| Some(c as u32)),
            color: Some(
                model.color.map_or_else(
                    || u32::from_str_radix("2B2D31", 16).unwrap_or_default(),
                    |c| c as u32
                )
            ),
            description: model.description,
            fields: model.fields.map_or_else(
                || vec![],
                |f| f.into_iter().map(EmbedField::from).collect()
            ), // Assuming you have a way to convert model fields to Vec<EmbedField>
            footer: model.footer_text.map(|text| EmbedFooter {
                text,
                icon_url: model.footer_icon_url,
                proxy_icon_url: None, // Proxy URLs are generally provided by Discord, not by the user
            }),
            image: model.image.map(|url| EmbedImage {
                url,
                proxy_url: None, // Proxy URLs are generally provided by Discord, not by the user
                height: None, // You may not have this information
                width: None, // You may not have this information
            }),
            kind: "rich".to_string(), // "rich" is a commonly used kind
            provider: None, // This is generally set by Discord when using links from known providers
            thumbnail: model.thumbnail.map(|url| EmbedThumbnail {
                url,
                proxy_url: None,
                height: None,
                width: None,
            }),

            title: model.title,
            url: model.url,
            video: None, // Assuming you don't have video information in your model
            timestamp: None,
        }
    }
}

impl From<DiscordEmbedField> for EmbedField {
    fn from(model: DiscordEmbedField) -> Self {
        EmbedField {
            name: model.name,
            value: model.value,
            inline: model.inline.map_or_else(
                || false,
                |i| i
            ),
        }
    }
}
