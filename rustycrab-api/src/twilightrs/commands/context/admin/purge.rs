use async_trait::async_trait;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::twilightrs::{
    commands::context::{
        ContextCommand,
        ParsedArg,
        ArgSpec,
        ArgType,
        context_command::GuildConfigModel,
    },
    discord_client::DiscordClient,
};

pub struct PurgeCommand;

#[async_trait]
impl ContextCommand for PurgeCommand {
    fn name(&self) -> &'static str {
        "purge"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["clear"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("amount of messages", ArgType::Number, false)] // User argument is not optional
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::MANAGE_MESSAGES]
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
        if let Some(ParsedArg::Number(amount)) = command_args.first() {
            // Ensure the amount is within a reasonable range
            let amount: u64 = (*amount).try_into().unwrap_or(100);
            let amount = amount.min(100).max(2); // Discord API limits bulk delete to 2-100 messages

            // Fetch the messages from the channel
            let messages = client.http
                .channel_messages(msg.channel_id)
                .limit(amount as u16)?.await?
                .model().await?
                .into_iter()
                .map(|message| message.id)
                .collect::<Vec<_>>();

            // Bulk delete messages
            if !messages.is_empty() {
                let _ = client.http.delete_messages(msg.channel_id, &messages)?.await;
            }
        }

        Ok(())
    }
}
