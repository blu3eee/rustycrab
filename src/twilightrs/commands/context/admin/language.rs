use async_trait::async_trait;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::{ DiscordClient, MessageContent },
    },
    queries::guild_config_queries::GuildConfigQueries,
    default_queries::DefaultSeaQueries,
    router::routes::bot_guild_configs::RequestUpdateConfig,
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
        let available_locales = vec!["en", "vn"];
        if let Some(ParsedArg::Arg(language)) = command_args.first() {
            if language.is_empty() || !available_locales.contains(&language.as_str()) {
                let _ = client.send_message(
                    msg.channel_id,
                    MessageContent::Text("Invalid language".to_string())
                ).await;
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
                    let _ = client.send_message(
                        msg.channel_id,
                        MessageContent::Text(
                            format!(
                                "Language updated successfully. New language: `{}`",
                                updated_config.prefix
                            )
                        )
                    ).await;
                } else {
                    let _ = client.send_message(
                        msg.channel_id,
                        MessageContent::Text("Failed to update language for this guild".to_string())
                    ).await;
                }
            }
        }

        Ok(())
    }
}
