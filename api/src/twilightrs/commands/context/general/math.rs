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
        vec![ArgSpec::new(ArgType::Text, false)]
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        vec![Box::new(LogicalCommand {}) as Box<dyn ContextCommand>]
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
                                description: message,
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

struct LogicalCommand {}

#[async_trait]
impl ContextCommand for LogicalCommand {
    fn name(&self) -> &'static str {
        "logical"
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        vec![Box::new(LogicalNestCommand {}) as Box<dyn ContextCommand>]
    }

    fn parent_command(&self) -> Option<Box<dyn ContextCommand>> {
        Some(Box::new(MathCommand {}) as Box<dyn ContextCommand>)
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new(ArgType::Text, false)]
    }

    // fn parent_command(&self) -> Option<Box<dyn ContextCommand>> {
    //     Some(Box::new(MathCommand {}) as Box<dyn ContextCommand>)
    // }

    async fn run(
        &self,
        client: &DiscordClient,
        _: &GuildConfigModel,
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
                    let _ = client.reply_message(
                        msg.channel_id,
                        msg.id,
                        MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                description: Some("Invalid math expression".to_string()),
                                ..Default::default()
                            }]
                        )
                    ).await;
                }
            }
        } else {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some("No math expression provided".to_string()),
                        ..Default::default()
                    }]
                )
            ).await;
        }

        Ok(())
    }
}

struct LogicalNestCommand {}

#[async_trait]
impl ContextCommand for LogicalNestCommand {
    fn name(&self) -> &'static str {
        "logical-nested"
    }
    async fn run(
        &self,
        client: &DiscordClient,
        _: &GuildConfigModel,
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
                    let _ = client.reply_message(
                        msg.channel_id,
                        msg.id,
                        MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                description: Some("Invalid math expression".to_string()),
                                ..Default::default()
                            }]
                        )
                    ).await;
                }
            }
        } else {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some("No math expression provided".to_string()),
                        ..Default::default()
                    }]
                )
            ).await;
        }

        Ok(())
    }
}
