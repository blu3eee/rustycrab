use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
};
pub struct MathCommand;

#[async_trait]
impl ContextCommand for MathCommand {
    fn name(&self) -> &'static str {
        "math"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["calc", "calculate"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("math expression", ArgType::Text, false)]
    }

    async fn run(
        &self,
        client: &DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ParsedArg::Text(expression)) = command_args.first() {
            match meval::eval_str(expression) {
                Ok(result) => {
                    let _ = client.reply_message(
                        msg.channel_id,
                        msg.id,
                        MessageContent::Text(result.to_string())
                    ).await;
                }
                Err(_) => {
                    let message = client.get_locale_string(
                        &config.locale,
                        "command-math-invalid",
                        None
                    );
                    let _ = client.reply_message(
                        msg.channel_id,
                        msg.id,
                        MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                description: Some(message),
                                ..Default::default()
                            }]
                        )
                    ).await;
                }
            }
        }

        Ok(())
    }
}
