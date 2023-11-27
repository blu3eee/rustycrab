use fluent::FluentArgs;

use sea_orm::DatabaseConnection;
use songbird::Songbird;
use twilight_cache_inmemory::{ InMemoryCache, model::CachedMessage };
use twilight_model::{
    channel::{ Message, message::{ embed::Embed, MessageFlags }, Channel },
    id::{ Id, marker::{ ChannelMarker, MessageMarker, UserMarker, GuildMarker, RoleMarker } },
    user::{ CurrentUser, User },
    http::interaction::{ InteractionResponse, InteractionResponseType, InteractionResponseData },
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

use super::{
    messages::DiscordEmbed,
    commands::context::context_command::GuildConfigModel,
    bot::voice_manager::VoiceManager,
};

use fluent::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use intl_memoizer::concurrent::IntlLangMemoizer;

pub enum MessageContent {
    Text(String),
    EmbedModels(Vec<EmbedModel>),
    TextAndEmbedModels(String, Vec<EmbedModel>),
    DiscordEmbeds(Vec<DiscordEmbed>),
    TextAndDiscordEmbeds(String, Vec<DiscordEmbed>),
    None,
}

pub type DiscordClient = Arc<DiscordClientRef>;

/// A reference to the Discord client, encapsulating various functionalities and states.
/// This structure provides access to database connections, HTTP client, in-memory cache,
/// and other shared resources necessary for bot operations.
pub struct DiscordClientRef {
    /// Connection to the database.
    pub db: DatabaseConnection,

    /// HTTP client for interacting with the Discord API.
    pub http: Arc<HttpClient>,

    /// In-memory cache of Discord entities.
    pub cache: Arc<InMemoryCache>,

    /// Record of deleted messages.
    pub deleted_messages: RwLock<HashMap<Id<ChannelMarker>, Vec<CachedMessage>>>,

    /// Record of users marked as 'away from keyboard' (AFK).
    pub afk_users: RwLock<HashMap<Id<GuildMarker>, HashMap<Id<UserMarker>, UserAfkStatus>>>,

    /// Manager for voice-related features.
    pub voice_manager: Arc<VoiceManager>,

    /// Localization bundles for multi-language support.
    pub bundles: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>>,

    /// Default localization bundle used when a specific guild's bundle is not found.
    pub default_bundle: FluentBundle<FluentResource, IntlLangMemoizer>,
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

impl DiscordClientRef {
    /// Constructs a new instance of `DiscordClientRef`.
    ///
    /// # Arguments
    ///
    /// * `db` - Connection to the database.
    /// * `http` - Arc-wrapped HTTP client for Discord API interactions.
    /// * `cache` - Arc-wrapped in-memory cache of Discord entities.
    /// * `songbird` - Arc-wrapped Songbird instance for voice functionality.
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordClientRef`.
    pub fn new(
        db: DatabaseConnection,
        http: Arc<HttpClient>,
        cache: Arc<InMemoryCache>,
        songbird: Arc<Songbird>
    ) -> Self {
        let mut bundles: HashMap<
            String,
            FluentBundle<FluentResource, IntlLangMemoizer>
        > = HashMap::new();
        for locale in vec!["en", "vn"] {
            let bundle = load_localization(locale);
            bundles.entry(locale.to_string()).or_insert(bundle);
        }

        DiscordClientRef {
            db,
            http,
            cache,
            voice_manager: Arc::new(VoiceManager::new(songbird)),
            deleted_messages: HashMap::new().into(),
            bundles,
            default_bundle: load_localization("en"),
            afk_users: HashMap::new().into(),
        }
    }

    /// Retrieves a localization string based on the provided locale and key.
    ///
    /// # Arguments
    ///
    /// * `locale` - Locale identifier (e.g., "en", "vn").
    /// * `key` - Key identifying the localization string.
    /// * `args` - Optional arguments for string formatting.
    ///
    /// # Returns
    ///
    /// Localized string or the key itself if the localization is not found.
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

    /// Fetches messages from a specified Discord channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - Reference to the channel entity.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of messages or an error.
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

    /// Retrieves configuration for a specific guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - ID of the guild.
    ///
    /// # Returns
    ///
    /// A `Result` containing the guild configuration model or an error.
    pub async fn get_guild(
        &self,
        guild_id: Id<GuildMarker>
    ) -> Result<twilight_model::guild::Guild, Box<dyn Error + Send + Sync>> {
        Ok(self.http.guild(guild_id).await?.model().await?)
    }

    /// Fetches the URL of a user's banner image.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the URL of the user's banner image or an error.
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

    /// Sends a message to a Discord channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - ID of the channel where the message will be sent.
    /// * `message_content` - Content of the message to be sent.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response of the message sent or an error.
    pub async fn send_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_content: MessageContent
    ) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
        self.send_discord_message(self.http.create_message(channel_id), message_content).await
    }

    /// Replies to a specific message in a Discord channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - ID of the channel where the reply will be sent.
    /// * `message_id` - ID of the message to which the reply is addressed.
    /// * `message_content` - Content of the reply.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response of the reply sent or an error.
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

    /// Edits an existing message in a Discord channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - ID of the channel containing the message.
    /// * `message_id` - ID of the message to be edited.
    /// * `message_content` - New content for the message.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response of the edited message or an error.
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

    pub async fn defer_button_interaction(
        &self,
        interaction: &Box<InteractionCreate>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
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

    pub async fn defer_ephemeral_interaction(
        &self,
        interaction: &Box<InteractionCreate>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.http.interaction(interaction.application_id).create_response(
            interaction.id,
            &interaction.token,
            &(InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    flags: Some(MessageFlags::EPHEMERAL),
                    ..Default::default()
                }),
            })
        ).await?;

        Ok(())
    }

    pub async fn defer_interaction(
        &self,
        interaction: &Box<InteractionCreate>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.http.interaction(interaction.application_id).create_response(
            interaction.id,
            &interaction.token,
            &(InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
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

    pub async fn is_user_in_same_channel_as_bot(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        let user_channel_id = self.cache
            .voice_state(user_id, guild_id)
            .and_then(|state| Some(state.channel_id()));

        let bot_user_id = self.http.current_user().await?.model().await?.id;

        let bot_channel_id = self.cache
            .voice_state(bot_user_id, guild_id)
            .and_then(|state| Some(state.channel_id()));

        Ok(user_channel_id == bot_channel_id)
    }
}

fn convert_embed_models(embed_models: Vec<EmbedModel>) -> Vec<Embed> {
    embed_models.into_iter().map(Embed::from).collect()
}

fn convert_discord_embeds(discord_embeds: Vec<DiscordEmbed>) -> Vec<Embed> {
    discord_embeds.into_iter().map(Embed::from).collect()
}
