use fluent::FluentArgs;

use sea_orm::DatabaseConnection;
use twilight_cache_inmemory::{ InMemoryCache, model::CachedMessage };
use twilight_model::{
    channel::{ Message, message::embed::Embed, Channel },
    id::{ Id, marker::{ ChannelMarker, MessageMarker, UserMarker, GuildMarker, RoleMarker } },
    user::{ CurrentUser, User },
    http::interaction::{ InteractionResponse, InteractionResponseType },
    gateway::payload::incoming::InteractionCreate,
    guild::Role,
};
use twilight_http::{ Client as HttpClient, Response, request::channel::message::CreateMessage };
use std::{ sync::{ Arc, RwLock }, error::Error, collections::HashMap };

use crate::{
    database::embed_info::Model as EmbedModel,
    locales::{ get_localized_string, load_localization },
    queries::guild_config_queries::GuildConfigQueries,
    unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries,
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

use fluent::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use intl_memoizer::concurrent::IntlLangMemoizer;

pub struct DiscordClient {
    pub db: DatabaseConnection,
    pub http: Arc<HttpClient>,
    pub cache: Arc<InMemoryCache>,
    pub deleted_messages: RwLock<HashMap<Id<ChannelMarker>, Vec<CachedMessage>>>,
    pub bundles: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>>,
    // Default bundle for new guilds or when specific guild bundle is not found
    pub default_bundle: FluentBundle<FluentResource, IntlLangMemoizer>,
    pub afk_users: RwLock<HashMap<Id<GuildMarker>, HashMap<Id<UserMarker>, UserAfkStatus>>>,
}

pub struct UserAfkStatus {
    pub message: Option<String>,
    pub since: u32,
    pub activities_count: u8,
    pub notify: Vec<Id<UserMarker>>,
}

impl UserAfkStatus {
    pub fn new(message: Option<String>, since: u32) -> Self {
        Self {
            message,
            since,
            activities_count: 0,
            notify: vec![],
        }
    }
}

impl DiscordClient {
    pub fn new(db: DatabaseConnection, http: Arc<HttpClient>, cache: Arc<InMemoryCache>) -> Self {
        let mut bundles: HashMap<
            String,
            FluentBundle<FluentResource, IntlLangMemoizer>
        > = HashMap::new();
        for locale in vec!["en", "vn"] {
            let bundle = load_localization(locale);
            bundles.entry(locale.to_string()).or_insert(bundle);
        }

        DiscordClient {
            db,
            http,
            cache,
            deleted_messages: HashMap::new().into(),
            bundles,
            default_bundle: load_localization("en"),
            afk_users: HashMap::new().into(),
        }
    }

    fn get_bundle(&self, locale: &str) -> &FluentBundle<FluentResource, IntlLangMemoizer> {
        if let Some(bundle) = self.bundles.get(locale) { bundle } else { &self.default_bundle }
    }

    pub fn get_locale_string(
        &self,
        locale: &str,
        key: &str,
        args: Option<&FluentArgs> // Use the same lifetime 'a here
    ) -> String {
        let bundle = self.get_bundle(locale);
        if let Some(result) = get_localized_string(bundle, key, args) {
            result
        } else {
            key.to_string()
        }
    }

    pub async fn fetch_messages(
        &self,
        channel: &Channel
    ) -> Result<Vec<Message>, Box<dyn Error + Send + Sync>> {
        if let Some(count) = channel.message_count {
            Ok(
                self.http
                    .channel_messages(channel.id)
                    .limit(count as u16)?.await?
                    .model().await?
            )
        } else {
            let mut msg_vec: Vec<Message> = Vec::new();

            let mut last_message_id: Option<Id<MessageMarker>> = None;

            loop {
                let fetched_messages = if let Some(id) = last_message_id {
                    self.http
                        .channel_messages(channel.id)
                        .before(id)
                        .limit(100)
                        ? // Use the maximum limit allowed by Discord
                        .await?.model().await?
                } else {
                    self.http
                        .channel_messages(channel.id)
                        .limit(100)
                        ? // Use the maximum limit allowed by Discord
                        .await?.model().await?
                };

                if fetched_messages.is_empty() {
                    break;
                }

                last_message_id = fetched_messages.last().map(|m| m.id);
                msg_vec.extend(fetched_messages);
            }

            Ok(msg_vec)
        }
    }

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

    pub async fn defer_interaction(
        &self,
        interaction: &Box<InteractionCreate>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.http.interaction(interaction.application_id).create_response(
            interaction.id,
            &interaction.token,
            &(InteractionResponse {
                kind: InteractionResponseType::DeferredUpdateMessage,
                data: None,
            })
        ).await?;

        Ok(())
    }

    pub async fn find_role(
        &self,
        guild_id: Id<GuildMarker>,
        role_arg: &str
    ) -> Result<Role, Box<dyn Error + Send + Sync>> {
        // Fetch all roles from the guild
        let roles = self.http.roles(guild_id).await?.model().await?;

        // Check if the argument is a direct ID
        if let Ok(role_id) = role_arg.parse::<u64>() {
            let role_id = Id::new(role_id);
            return roles
                .into_iter()
                .find(|role| role.id == role_id)
                .ok_or_else(|| "Role not found".into());
        }

        // Check if the argument is a role mention
        if role_arg.starts_with("<@&") && role_arg.ends_with(">") {
            if let Ok(role_id) = role_arg[3..role_arg.len() - 1].parse::<u64>() {
                let role_id = Id::new(role_id);
                return roles
                    .into_iter()
                    .find(|role| role.id == role_id)
                    .ok_or_else(|| "Role not found".into());
            }
        }

        // Find the role by name
        roles
            .into_iter()
            .find(
                |role|
                    role.name.eq_ignore_ascii_case(role_arg) ||
                    role.name.to_ascii_lowercase().contains(&role_arg.to_ascii_lowercase())
            )
            .ok_or_else(|| "Role not found".into())
    }

    pub async fn user_has_role(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Fetch the member
        let member = self.http.guild_member(guild_id, user_id).await?.model().await?;

        // Check if the member has the role
        Ok(member.roles.contains(&role_id))
    }
}

fn convert_embed_models(embed_models: Vec<EmbedModel>) -> Vec<Embed> {
    embed_models.into_iter().map(Embed::from).collect()
}

fn convert_discord_embeds(discord_embeds: Vec<DiscordEmbed>) -> Vec<Embed> {
    discord_embeds.into_iter().map(Embed::from).collect()
}
