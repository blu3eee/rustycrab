use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    twilightrs::{
        commands::context::{
            ContextCommand,
            ParsedArg,
            context_command::GuildConfigModel,
            ArgSpec,
            ArgType,
        },
        discord_client::DiscordClient,
        utils::reply_command,
    },
    queries::auto_responses_queries::AutoResponsesQueries,
    default_queries::DefaultSeaQueries,
};

use super::AutoResCommand;
pub struct DeleteAutoResponseCommand;

#[async_trait]
impl ContextCommand for DeleteAutoResponseCommand {
    fn name(&self) -> &'static str {
        "delete"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["del", "remove"]
    }
    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("trigger", ArgType::Text, false)]
    }

    fn parent_command(&self) -> Option<Box<dyn ContextCommand>> {
        Some(Box::new(AutoResCommand {}) as Box<dyn ContextCommand>)
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let bot = client.get_bot().await?;

        let mut args: FluentArgs<'_> = FluentArgs::new();
        let (key, color) = if
            let ParsedArg::Text(trigger) = command_args.first().ok_or("invalid command")?
        {
            args.set("trigger", trigger.to_string());
            if
                let Ok(autores) = AutoResponsesQueries::find_by_trigger(
                    &client.db,
                    &bot.id.to_string(),
                    &guild_id.to_string(),
                    trigger
                ).await
            {
                if let Ok(_) = AutoResponsesQueries::delete_by_id(&client.db, autores.id).await {
                    ("autores-deleted", ColorResolvables::Green)
                } else {
                    ("autores-delete-failed", ColorResolvables::Red)
                }
            } else {
                ("autores-notfound", ColorResolvables::Yellow)
            }
        } else {
            ("command-invalid", ColorResolvables::Red)
        };

        let _ = reply_command(&client, &config, &msg, key, Some(args), color).await;

        Ok(())
    }
}
