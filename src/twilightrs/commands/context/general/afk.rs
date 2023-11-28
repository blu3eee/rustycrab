use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    twilightrs::{
        commands::context::{
            ContextCommand,
            ParsedArg,
            ArgSpec,
            ArgType,
            context_command::GuildConfigModel,
        },
        discord_client::DiscordClient,
        bot::afk::UserAfkStatus,
    },
    utilities::utils::current_unix_timestamp,
};

pub struct AfkCommand;

#[async_trait]
impl ContextCommand for AfkCommand {
    fn name(&self) -> &'static str {
        "afk"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("message from you", ArgType::Text, true)] // User argument is optional
    }
    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.unwrap(); // Assuming command is used in a guild
        let user_id = msg.author.id;

        // Initiate the args for fluent bundle
        let mut args = FluentArgs::new();
        args.set("user", format!("<@{}>", user_id));

        // Extract optional AFK message
        let afk_message = command_args.first().and_then(|arg| {
            if let ParsedArg::Text(message) = arg { Some(message.clone()) } else { None }
        });

        // Get the current timestamp
        let since = current_unix_timestamp()?; // Implement this function to get the current time

        let guild_member = client.http.guild_member(guild_id, user_id).await?.model().await?;
        // Update or set the AFK status
        {
            let mut afk_users = client.afk_users.write().unwrap();
            let user_afk_status = afk_users
                .entry(guild_id)
                .or_default()
                .entry(user_id)
                .or_insert(UserAfkStatus::new(None, since));

            user_afk_status.message = afk_message;
            user_afk_status.since = since;
        } // The lock is dropped here

        let current_name = if let Some(nickname) = guild_member.nick {
            nickname
        } else {
            guild_member.user.name
        };

        if !current_name.trim().starts_with("[AFK]") {
            let _ = client.http
                .update_guild_member(guild_id, user_id)
                .nick(Some(&format!("[AFK] {}", current_name)))?.await;
        }

        // Send confirmation message
        let message = client.get_locale_string(&config.locale, "command-afk-success", Some(&args));
        client.http.create_message(msg.channel_id).content(&message)?.await?;

        Ok(())
    }
}
