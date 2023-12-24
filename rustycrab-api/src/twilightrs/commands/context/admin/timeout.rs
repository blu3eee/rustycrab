use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    guild::Permissions,
    util::Timestamp,
};
use std::{ error::Error, time::SystemTime };

use crate::twilightrs::{
    commands::context::{
        ContextCommand,
        ParsedArg,
        ArgSpec,
        ArgType,
        context_command::GuildConfigModel,
    },
    discord_client::DiscordClient,
    utils::send_command_response,
};

use std::time::Duration;

pub struct TimeoutMemberCommand;

#[async_trait]
impl ContextCommand for TimeoutMemberCommand {
    fn name(&self) -> &'static str {
        "timeout"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["mute"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![
            ArgSpec::new("users", ArgType::Users, false),
            ArgSpec::new("duration", ArgType::Number, false), // Duration in minutes
            ArgSpec::new("reason", ArgType::Text, true)
        ]
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::MODERATE_MEMBERS]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        if let Some(ParsedArg::Users(users)) = command_args.first() {
            if let Some(ParsedArg::Number(duration)) = command_args.get(1) {
                let timeout_duration = Duration::from_secs(*duration as u64); // Convert minutes to seconds
                let timeout_end = SystemTime::now() + timeout_duration;

                // Convert SystemTime to Timestamp
                let timestamp = Timestamp::from_secs(
                    timeout_end.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64
                )?;
                for user in users {
                    let mut args = FluentArgs::new();
                    args.set("user", format!("<@{}>", user.id.to_string()));
                    args.set("duration", format!("{}", duration));

                    let (key, color) = match
                        client.http
                            .update_guild_member(guild_id, user.id)
                            .communication_disabled_until(Some(timestamp))?.await
                    {
                        Ok(_) => { ("command-timeout-success", ColorResolvables::Green) }
                        Err(e) => {
                            args.set("err", format!("{}", e));
                            ("command-timeout-fail", ColorResolvables::Red)
                        }
                    };
                    let _ = send_command_response(
                        &client,
                        &config,
                        &msg,
                        key,
                        Some(args),
                        color
                    ).await;
                }
            }
        }

        Ok(())
    }
}
