use async_trait::async_trait;
use fluent_bundle::FluentArgs;
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
    utilities::utils::{ ColorResolvables, parse_colorhex },
    default_queries::DefaultSeaQueries,
    router::routes::{
        auto_responses::RequestUpdateAutoResponse,
        RequestCreateUpdateMessage,
        RequestCreateUpdateEmbed,
    },
};

use super::{ AutoResCommand, utils::split_trigger_and_value };
pub struct ColorUpdateAutoResCommand;

#[async_trait]
impl ContextCommand for ColorUpdateAutoResCommand {
    fn name(&self) -> &'static str {
        "color"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("trigger | color hex (#fafafa)", ArgType::Text, false)]
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

        let (trigger, color) = split_trigger_and_value(command_args)?;

        let mut args: FluentArgs<'_> = FluentArgs::new();
        args.set("trigger", trigger.to_string());

        let autores = AutoResponsesQueries::find_by_trigger(
            &client.db,
            &bot.id.to_string(),
            &guild_id.to_string(),
            trigger.as_str()
        ).await.map_err(|_| client.get_locale_string(&config.locale, "autores-notfound", None))?;

        let color = parse_colorhex(&color).ok_or(
            client.get_locale_string(&config.locale, "invalid-color", None)
        )?;

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
                            color: Some(color.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).await
        {
            embed.description = Some(format!("Color hex: {}", color));
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
