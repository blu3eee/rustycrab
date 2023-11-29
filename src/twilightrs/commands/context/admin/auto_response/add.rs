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
    utilities::utils::ColorResolvables,
    default_queries::DefaultSeaQueries,
    router::routes::auto_responses::RequestCreateAutoResponse,
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
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let bot = client.get_bot().await?;

        let (trigger, response) = split_trigger_and_value(command_args)?;

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
                        response_data: if response.is_empty() {
                            None
                        } else {
                            Some(crate::router::routes::RequestCreateUpdateMessage {
                                r#type: Some("Message".to_string()),
                                content: Some(response.to_string()),
                                embed: None,
                            })
                        },
                    }
                ).await
            {
                ("autores-create-success", ColorResolvables::Green)
            } else {
                ("autores-create-failed", ColorResolvables::Red)
            }
        };

        let _ = reply_command(&client, &config, &msg, key, None, color).await;

        Ok(())
    }
}
