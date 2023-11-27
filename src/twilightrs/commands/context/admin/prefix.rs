use async_trait::async_trait;
use fluent_bundle::FluentArgs;
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

pub struct ChangePrefixCommand;

#[async_trait]
impl ContextCommand for ChangePrefixCommand {
    fn name(&self) -> &'static str {
        "changeprefix"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["prefix", "chprefix"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("new prefix", ArgType::Arg, false)] // User argument is optional
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
        if let Some(ParsedArg::Arg(new_prefix)) = command_args.first() {
            if new_prefix.is_empty() {
                let message = client.get_locale_string(
                    &config.locale,
                    "command-prefix-invalid",
                    None
                );
                let _ = client.send_message(msg.channel_id, MessageContent::Text(message)).await;
            } else {
                let update_result = GuildConfigQueries::update_by_id(
                    &client.db,
                    config.id,
                    RequestUpdateConfig {
                        prefix: Some(new_prefix.clone()),
                        ..Default::default()
                    }
                ).await;

                if let Ok(updated_config) = update_result {
                    let mut args = FluentArgs::new();
                    args.set("prefix", updated_config.prefix);
                    let message = client.get_locale_string(
                        &config.locale,
                        "command-prefix-success",
                        Some(&args)
                    );

                    let _ = client.send_message(
                        msg.channel_id,
                        MessageContent::Text(message)
                    ).await;
                } else {
                    let _ = client.send_message(
                        msg.channel_id,
                        MessageContent::Text("Failed to update prefix for this guild".to_string())
                    ).await;
                }
            }
        }

        Ok(())
    }
}
