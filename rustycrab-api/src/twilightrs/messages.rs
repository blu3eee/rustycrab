use std::{ time::{ SystemTime, UNIX_EPOCH }, sync::Arc };

use twilight_http::Client;
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
    id::{ Id, marker::{ GuildMarker, UserMarker } },
};

use crate::{
    database::embed_info::Model as EmbedModel,
    utilities::utils::process_placeholders_sync,
};

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

    pub fn is_empty(&self) -> bool {
        (self.fields.is_none() || self.fields.as_ref().unwrap().len() == 0) &&
            (self.title.is_none() || self.title.as_ref().unwrap().is_empty()) &&
            (self.description.is_none() || self.description.as_ref().unwrap().is_empty()) &&
            (self.footer_text.is_none() || self.footer_text.as_ref().unwrap().is_empty()) &&
            (self.author_name.is_none() || self.author_name.as_ref().unwrap().is_empty()) &&
            (self.thumbnail.is_none() || self.thumbnail.as_ref().unwrap().is_empty()) &&
            (self.image.is_none() || self.image.as_ref().unwrap().is_empty())
    }

    pub async fn to_embed(
        self,
        http: &Arc<Client>,
        guild_id: Option<Id<GuildMarker>>,
        user_id: Option<Id<UserMarker>>
    ) -> Embed {
        let guild = if let Some(guild_id) = guild_id {
            if let Ok(guild) = http.guild(guild_id).await {
                if let Ok(guild) = guild.model().await { Some(guild) } else { None }
            } else {
                None
            }
        } else {
            None
        };

        let user = if let Some(user_id) = user_id {
            if let Ok(user) = http.user(user_id).await {
                if let Ok(user) = user.model().await { Some(user) } else { None }
            } else {
                None
            }
        } else {
            None
        };

        Embed {
            author: self.author_name.map(|name| EmbedAuthor {
                name: process_placeholders_sync(name, &guild, &user),
                icon_url: self.author_icon_url.map(|url|
                    process_placeholders_sync(url, &guild, &user)
                ),
                proxy_icon_url: None,
                url: None, // If you have a corresponding URL in EmbedModel, use it here
            }),
            // color: model.color.and_then(|c| Some(c as u32)),
            color: Some(
                self.color.map_or_else(
                    || u32::from_str_radix("2B2D31", 16).unwrap_or_default(),
                    |c| c as u32
                )
            ),
            description: self.description.map(|text|
                process_placeholders_sync(text, &guild, &user)
            ),
            fields: self.fields.map_or_else(
                || vec![],
                |f| f.into_iter().map(EmbedField::from).collect()
            ), // Assuming you have a way to convert model fields to Vec<EmbedField>
            footer: self.footer_text.map(|text| EmbedFooter {
                text: process_placeholders_sync(text, &guild, &user),
                icon_url: self.footer_icon_url.map(|text|
                    process_placeholders_sync(text, &guild, &user)
                ),
                proxy_icon_url: None, // Proxy URLs are generally provided by Discord, not by the user
            }),
            image: self.image.map(|url| EmbedImage {
                url: process_placeholders_sync(url, &guild, &user),
                proxy_url: None, // Proxy URLs are generally provided by Discord, not by the user
                height: None, // You may not have this information
                width: None, // You may not have this information
            }),
            kind: "rich".to_string(), // "rich" is a commonly used kind
            provider: None, // This is generally set by Discord when using links from known providers
            thumbnail: self.thumbnail.map(|url| EmbedThumbnail {
                url: process_placeholders_sync(url, &guild, &user),
                proxy_url: None,
                height: None,
                width: None,
            }),
            title: self.title.map(|text| process_placeholders_sync(text, &guild, &user)),
            url: self.url.map(|text| process_placeholders_sync(text, &guild, &user)),
            video: None, // Assuming you don't have video information in your model
            timestamp: self.timestamp
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

    pub async fn process_placeholders(
        self,
        http: &Arc<Client>,
        guild_id: Option<Id<GuildMarker>>,
        user_id: Option<Id<UserMarker>>
    ) -> Self {
        let guild = if let Some(guild_id) = guild_id {
            if let Ok(guild) = http.guild(guild_id).await {
                if let Ok(guild) = guild.model().await { Some(guild) } else { None }
            } else {
                None
            }
        } else {
            None
        };

        let user = if let Some(user_id) = user_id {
            if let Ok(user) = http.user(user_id).await {
                if let Ok(user) = user.model().await { Some(user) } else { None }
            } else {
                None
            }
        } else {
            None
        };

        Self {
            author_name: self.author_name.map(|name|
                process_placeholders_sync(name, &guild, &user)
            ),
            author_icon_url: self.author_icon_url.map(|url|
                process_placeholders_sync(url, &guild, &user)
            ),
            // color: model.color.and_then(|c| Some(c as u32)),
            color: Some(
                self.color.map_or_else(
                    || u32::from_str_radix("2B2D31", 16).unwrap_or_default(),
                    |c| c as u32
                )
            ),
            description: self.description.map(|text|
                process_placeholders_sync(text, &guild, &user)
            ),
            footer_text: self.footer_text.map(|text|
                process_placeholders_sync(text, &guild, &user)
            ),
            image: self.image.map(|url| process_placeholders_sync(url, &guild, &user)),
            title: self.title.map(|text| process_placeholders_sync(text, &guild, &user)),
            url: self.url.map(|text| process_placeholders_sync(text, &guild, &user)),
            thumbnail: self.thumbnail.map(|url| process_placeholders_sync(url, &guild, &user)),
            footer_icon_url: self.footer_icon_url.map(|url|
                process_placeholders_sync(url, &guild, &user)
            ),
            fields: self.fields.map(|fields|
                fields
                    .iter()
                    .map(|field| DiscordEmbedField {
                        name: process_placeholders_sync(field.name.clone(), &guild, &user),
                        value: process_placeholders_sync(field.value.clone(), &guild, &user),
                        inline: field.inline,
                    })
                    .collect::<Vec<DiscordEmbedField>>()
            ),
            timestamp: self.timestamp,
        }
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
                text: text,
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
