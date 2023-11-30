use async_trait::async_trait;
use rustycrab_model::response::bot_guild_config::RequestUpdateConfig;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::{
    twilightrs::{
        commands::context::{
            ContextCommand,
            ParsedArg,
            ArgSpec,
            ArgType,
            context_command::GuildConfigModel,
        },
        discord_client::{ DiscordClient, MessageContent },
    },
    queries::guild_config_queries::GuildConfigQueries,
    default_queries::DefaultSeaQueries,
};
pub struct ChangeLanguageCommand;

#[async_trait]
impl ContextCommand for ChangeLanguageCommand {
    fn name(&self) -> &'static str {
        "language"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["locale", "changelanguage"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("language (en/vn)", ArgType::Arg, false)]
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::ADMINISTRATOR]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let _ = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        let available_locales = vec!["en", "vn"];
        if let Some(ParsedArg::Arg(language)) = command_args.first() {
            let content = if language.is_empty() || !available_locales.contains(&language.as_str()) {
                "Invalid language".to_string()
            } else {
                let update_result = GuildConfigQueries::update_by_id(
                    &client.db,
                    config.id,
                    RequestUpdateConfig {
                        locale: Some(language.clone()),
                        ..Default::default()
                    }
                ).await;

                if let Ok(updated_config) = update_result {
                    format!(
                        "Language updated successfully. New language: `{}`",
                        updated_config.prefix
                    )
                } else {
                    "Failed to update language for this guild".to_string()
                }
            };

            let _ = client.send_message(msg.channel_id, MessageContent::Text(content)).await;
        }

        Ok(())
    }
}
