use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::{ color::ColorResolvables, response::bot_guild_config::RequestUpdateConfig };
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
        discord_client::DiscordClient,
        utils::reply_command,
    },
    queries::guild_config_queries::GuildConfigQueries,
    default_queries::DefaultSeaQueries,
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
        let _ = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        if let Some(ParsedArg::Arg(new_prefix)) = command_args.first() {
            let mut args = FluentArgs::new();
            let (key, color) = if new_prefix.is_empty() {
                ("command-prefix-invalid", ColorResolvables::Red)
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
                    args.set("prefix", updated_config.prefix);

                    ("command-prefix-success", ColorResolvables::Green)
                } else {
                    ("command-prefix-failed", ColorResolvables::Red)
                }
            };

            let _ = reply_command(&client, &config, &msg, key, Some(args), color).await;
        }

        Ok(())
    }
}
