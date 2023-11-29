use async_trait::async_trait;
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
    utilities::utils::{ ColorResolvables, validate_image_url },
    default_queries::DefaultSeaQueries,
    router::routes::{
        auto_responses::RequestUpdateAutoResponse,
        RequestCreateUpdateMessage,
        RequestCreateUpdateEmbed,
    },
};

use super::{ AutoResCommand, utils::split_trigger_and_value };
pub struct ImageUpdateAutoResCommand;

#[async_trait]
impl ContextCommand for ImageUpdateAutoResCommand {
    fn name(&self) -> &'static str {
        "image"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["i", "iurl"]
    }
    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("url", ArgType::Arg, false)]
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

        let (trigger, url) = split_trigger_and_value(command_args)?;

        let autores = AutoResponsesQueries::find_by_trigger(
            &client.db,
            &bot.id.to_string(),
            &guild_id.to_string(),
            trigger.as_str()
        ).await?;
        if !validate_image_url(url.as_str()) {
            return Err(
                client.get_locale_string(&config.locale, "autores-invalid-url", None).into()
            );
        }

        let (key, color) = if
            let Ok(_) = AutoResponsesQueries::update_by_id(
                &client.db,
                autores.id,
                RequestUpdateAutoResponse {
                    response_data: Some(RequestCreateUpdateMessage {
                        embed: Some(RequestCreateUpdateEmbed {
                            image: Some(url.to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).await
        {
            ("autores-update-success", ColorResolvables::Green)
        } else {
            ("autores-update-failed", ColorResolvables::Red)
        };

        let _ = reply_command(&client, &config, &msg, key, None, color).await;

        Ok(())
    }
}
