use std::time::{ SystemTime, UNIX_EPOCH };

use sea_orm::DatabaseConnection;
use twilight_model::{
    channel::{
        Message,
        message::{ Embed, embed::{ EmbedAuthor, EmbedFooter, EmbedImage, EmbedThumbnail } },
    },
    id::{ Id, marker::{ ChannelMarker, MessageMarker } },
    util::Timestamp,
};
use twilight_http::{ Client as HttpClient, Response };
use std::{ sync::Arc, error::Error, time::Instant };

use crate::utilities::app_error::AppError;
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
    Embeds(Vec<DiscordEmbed>),
    TextAndEmbeds(String, Vec<DiscordEmbed>),
    None,
}

#[derive(Clone)]
pub struct DiscordClient {
    pub db: DatabaseConnection,
    // pub shard: Shard,
    pub http: Arc<HttpClient>,
    // pub cache: InMemoryCache,
}

impl DiscordClient {
    pub async fn send_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        let create_message_result: Result<Response<Message>, twilight_http::Error> = match
            message_content
        {
            MessageContent::Text(text) => {
                self.http.create_message(channel_id).content(&text)?.await
            }
            MessageContent::Embeds(embed) => {
                // Convert your DiscordEmbed into a Twilight Embed here
                // let twilight_embed = embed_to_twilight_embed(embed);
                // self.http.create_message(channel_id)
                //     .embeds(&[twilight_embed]) // Assuming twilight_embed is the converted embed
                //     .exec()
                //     .await
                todo!() // Implement your embed conversion and sending logic
            }
            MessageContent::TextAndEmbeds(_, _) => todo!(),
            MessageContent::None => todo!(),
            // ... handle other variants ...
        };

        create_message_result.map_err(|e| e.into())
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
            color: model.color.and_then(|c|
                u32::from_str_radix(&c.trim_start_matches('#'), 16).ok()
            ),
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
