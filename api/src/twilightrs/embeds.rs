use std::time::{ SystemTime, UNIX_EPOCH };

use twilight_model::{
    channel::message::embed::{
        Embed,
        EmbedAuthor,
        EmbedFooter,
        EmbedImage,
        EmbedThumbnail,
        EmbedField,
    },
    util::Timestamp,
};

use crate::database::embed_info::Model as EmbedModel;

#[derive(Debug, Clone, Default)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub footer_text: Option<String>,
    pub footer_icon_url: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub author_name: Option<String>,
    pub author_icon_url: Option<String>,
    pub fields: Option<Vec<DiscordEmbedField>>,
    pub timestamp: Option<bool>,
}

impl DiscordEmbed {
    pub fn new() -> Self {
        DiscordEmbed { ..Default::default() }
    }
}

#[derive(Debug, Clone)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
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
            timestamp: model.timestamp
                .map(|t| {
                    if t {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        Timestamp::from_micros(since_the_epoch.as_micros() as i64).ok()
                    } else {
                        None
                    }
                })
                .flatten(),
        }
    }
}

impl From<DiscordEmbedField> for EmbedField {
    fn from(model: DiscordEmbedField) -> Self {
        EmbedField {
            name: model.name,
            value: model.value,
            inline: model.inline,
        }
    }
}

impl From<EmbedModel> for DiscordEmbed {
    fn from(model: EmbedModel) -> Self {
        DiscordEmbed {
            title: model.title,
            description: model.description,
            url: model.url,
            color: Some(
                u32
                    ::from_str_radix(
                        &model.color
                            .unwrap_or_else(|| "2B2D31".to_string())
                            .trim_start_matches('#'),
                        16
                    )
                    .unwrap_or_default()
            ),
            footer_text: model.footer,
            footer_icon_url: model.footer_url,
            image: model.image,
            thumbnail: model.thumbnail,
            author_name: model.author,
            author_icon_url: model.author_url,
            // Assuming `fields` in `EmbedModel` can be converted to `Vec<DiscordEmbedField>`
            fields: None,
            timestamp: model.timestamp.map(|_| true), // Adjust this based on your timestamp logic
        }
    }
}

impl From<EmbedModel> for Embed {
    fn from(model: EmbedModel) -> Self {
        let discord_embed: DiscordEmbed = model.into();
        discord_embed.into()
    }
}
