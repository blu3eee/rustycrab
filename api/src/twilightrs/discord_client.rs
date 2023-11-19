use fluent_bundle::FluentArgs;
use sea_orm::DatabaseConnection;
use twilight_cache_inmemory::{ InMemoryCache, model::CachedMessage };
use twilight_model::{
    channel::{ Message, message::embed::Embed },
    id::{ Id, marker::{ ChannelMarker, MessageMarker, UserMarker, GuildMarker } },
    user::{ CurrentUser, User },
};
use twilight_http::{ Client as HttpClient, Response, request::channel::message::CreateMessage };
use std::{ sync::{ Arc, RwLock }, error::Error, collections::HashMap };

use crate::{
    database::embed_info::Model as EmbedModel,
    locales::{ get_localized_string, load_localization },
    queries::guild_config_queries::GuildConfigQueries,
    bot_guild_entity_queries::BotGuildEntityQueries,
};

use super::{ messages::DiscordEmbed, commands::context::context_command::GuildConfigModel };
pub enum MessageContent {
    Text(String),
    EmbedModels(Vec<EmbedModel>),
    TextAndEmbedModels(String, Vec<EmbedModel>),
    DiscordEmbeds(Vec<DiscordEmbed>),
    TextAndDiscordEmbeds(String, Vec<DiscordEmbed>),
    None,
}

pub struct DiscordClient {
    pub db: DatabaseConnection,
    pub http: Arc<HttpClient>,
    pub cache: Arc<InMemoryCache>,
    pub deleted_messages: RwLock<HashMap<Id<ChannelMarker>, Vec<CachedMessage>>>,
}

pub enum GuildIdentifier {
    GuildId(Id<GuildMarker>),
    Config(GuildConfigModel),
}

impl DiscordClient {
    pub async fn get_guild(
        &self,
        guild_id: Option<Id<GuildMarker>>
    ) -> Result<Option<twilight_model::guild::Guild>, Box<dyn Error + Send + Sync>> {
        if let Some(id) = guild_id {
            Ok(Some(self.http.guild(id).await?.model().await?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_guild_config(
        &self,
        guild_id: &Id<GuildMarker>
    ) -> Result<GuildConfigModel, Box<dyn Error + Send + Sync>> {
        let bot_id: String = self.http.current_user().await?.model().await?.id.get().to_string();
        let guild_id: String = guild_id.get().to_string();

        Ok(GuildConfigQueries::find_by_discord_ids(&self.db, &bot_id, &guild_id).await?)
    }

    pub fn get_locale_string(
        &self,
        locale: &str,
        key: &str,
        args: Option<&FluentArgs> // Use the same lifetime 'a here
    ) -> Option<String> {
        let bundle = load_localization(&locale);
        get_localized_string(&bundle, key, args)
    }

    pub async fn get_locale(&self, guild_identifier: GuildIdentifier) -> String {
        match guild_identifier {
            GuildIdentifier::Config(config) => config.locale.clone(),
            GuildIdentifier::GuildId(guild_id) => {
                if let Ok(config) = self.get_guild_config(&guild_id).await {
                    config.locale
                } else {
                    "en".to_string()
                }
            }
        }
    }

    pub async fn get_user_banner_url(
        &self,
        user_id: Id<UserMarker>
    ) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        // Fetch user from Discord API
        let user: User = self.http.user(user_id).await?.model().await?;

        // Construct the banner URL if available
        if let Some(banner) = user.banner {
            let format = if banner.to_string().starts_with("a_") { "gif" } else { "png" };
            let banner_url = format!(
                "https://cdn.discordapp.com/banners/{}/{}.{}?size=512",
                user_id,
                banner,
                format
            );
            Ok(Some(banner_url))
        } else {
            Ok(None)
        }
    }

    pub async fn get_bot(&self) -> Result<CurrentUser, Box<dyn Error + Send + Sync>> {
        Ok(self.http.current_user().await?.model().await?)
    }

    async fn send_discord_message(
        &self,
        create_message: CreateMessage<'_>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        match message_content {
            MessageContent::Text(text) => { Ok(create_message.content(&text)?.await?) }
            MessageContent::EmbedModels(embeds) => {
                Ok(create_message.embeds(&convert_embed_models(embeds))?.await?)
            }
            MessageContent::TextAndEmbedModels(text, embeds) => {
                Ok(create_message.content(&text)?.embeds(&convert_embed_models(embeds))?.await?)
            }
            MessageContent::DiscordEmbeds(embeds) => {
                Ok(create_message.embeds(&convert_discord_embeds(embeds))?.await?)
            }
            MessageContent::TextAndDiscordEmbeds(text, embeds) => {
                Ok(create_message.content(&text)?.embeds(&convert_discord_embeds(embeds))?.await?)
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
                Ok(message_update.embeds(Some(&convert_embed_models(embeds)))?.await?)
            }
            MessageContent::TextAndEmbedModels(text, embeds) => {
                Ok(
                    message_update
                        .content(Some(&text))?
                        .embeds(Some(&convert_embed_models(embeds)))?.await?
                )
            }
            MessageContent::DiscordEmbeds(embeds) => {
                Ok(message_update.embeds(Some(&convert_discord_embeds(embeds)))?.await?)
            }
            MessageContent::TextAndDiscordEmbeds(text, embeds) => {
                Ok(
                    message_update
                        .content(Some(&text))?
                        .embeds(Some(&convert_discord_embeds(embeds)))?.await?
                )
            }
            MessageContent::None => {
                // Handle case where no content is provided (might clear the content or do nothing)
                Err("No content provided for edit".into())
            }
        }
    }
}

fn convert_embed_models(embed_models: Vec<EmbedModel>) -> Vec<Embed> {
    embed_models.into_iter().map(Embed::from).collect()
}

fn convert_discord_embeds(discord_embeds: Vec<DiscordEmbed>) -> Vec<Embed> {
    discord_embeds.into_iter().map(Embed::from).collect()
}
