use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::{
    color::ColorResolvables,
    response::{
        auto_response::RequestCreateAutoResponse,
        discord_message::RequestCreateUpdateMessage,
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
        discord_client::DiscordClient,
        utils::reply_command,
    },
    queries::auto_responses_queries::AutoResponsesQueries,
    default_queries::DefaultSeaQueries,
};

use super::{ AutoResCommand, utils::split_trigger_and_value };
pub struct AddAutoResponseCommand;

#[async_trait]
impl ContextCommand for AddAutoResponseCommand {
    fn name(&self) -> &'static str {
        "add"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("trigger | response", ArgType::Text, false)]
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

        let (trigger, response) = split_trigger_and_value(command_args)?;

        let mut args: FluentArgs<'_> = FluentArgs::new();
        args.set("trigger", trigger.to_string());

        let (key, color) = if
            let Ok(_) = AutoResponsesQueries::find_by_trigger(
                &client.db,
                &bot.id.to_string(),
                &guild_id.to_string(),
                trigger.as_str()
            ).await
        {
            ("autores-existed", ColorResolvables::Yellow)
        } else {
            if
                let Ok(_) = AutoResponsesQueries::create_entity(
                    &client.db,
                    RequestCreateAutoResponse {
                        bot_discord_id: bot.id.to_string(),
                        guild_discord_id: guild_id.to_string(),
                        trigger: trigger.to_string(),
                        response_data: RequestCreateUpdateMessage {
                            r#type: Some("Embed and Text".to_string()),
                            content: Some(response.to_string()),
                            embed: None,
                        },
                    }
                ).await
            {
                ("autores-created", ColorResolvables::Green)
            } else {
                ("autores-create-failed", ColorResolvables::Red)
            }
        };

        let _ = reply_command(&client, &config, &msg, key, Some(args), color).await;

        Ok(())
    }
}
