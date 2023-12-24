use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::{
    color::ColorResolvables,
    response::{
        auto_response::RequestUpdateAutoResponse,
        discord_message::{ RequestCreateUpdateMessage, RequestCreateUpdateEmbed },
    },
};
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
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    queries::auto_responses_queries::AutoResponsesQueries,
    default_queries::DefaultSeaQueries,
};

use super::{ AutoResCommand, utils::split_trigger_and_value };
pub struct ContentUpdateAutoResCommand;

#[async_trait]
impl ContextCommand for ContentUpdateAutoResCommand {
    fn name(&self) -> &'static str {
        "content"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("trigger | text", ArgType::Text, false)]
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
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        let bot = client.get_bot().await?;

        let (trigger, content) = split_trigger_and_value(command_args)?;

        let mut args: FluentArgs<'_> = FluentArgs::new();
        args.set("trigger", trigger.to_string());

        let autores = AutoResponsesQueries::find_by_trigger(
            &client.db,
            &bot.id.to_string(),
            &guild_id.to_string(),
            trigger.as_str()
        ).await.map_err(|_| client.get_locale_string(&config.locale, "autores-notfound", None))?;

        let mut embed = DiscordEmbed {
            ..Default::default()
        };

        let (key, color) = if
            let Ok(_) = AutoResponsesQueries::update_by_id(
                &client.db,
                autores.id,
                RequestUpdateAutoResponse {
                    response_data: Some(RequestCreateUpdateMessage {
                        embed: Some(RequestCreateUpdateEmbed {
                            description: Some(content.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).await
        {
            embed.description = Some(content);
            ("autores-updated", ColorResolvables::Green)
        } else {
            ("autores-update-failed", ColorResolvables::Red)
        };

        embed.color = Some(color.as_u32());
        embed.author_name = Some(client.get_locale_string(&config.locale, key, Some(&args)));

        let _ = client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(vec![embed])
        ).await;

        Ok(())
    }
}
