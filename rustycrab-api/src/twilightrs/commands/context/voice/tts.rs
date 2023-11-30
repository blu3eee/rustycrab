use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{
        context_command::{ ContextCommand, GuildConfigModel },
        ParsedArg,
        ArgSpec,
        ArgType,
    },
    discord_client::DiscordClient,
};

pub struct TtsCommand {}

#[async_trait]
impl ContextCommand for TtsCommand {
    fn name(&self) -> &'static str {
        "tts"
    }

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("text to speech", ArgType::Text, false)]
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::new()
    }

    async fn run(
        &self,
        client: DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
}

impl TtsCommand {}
