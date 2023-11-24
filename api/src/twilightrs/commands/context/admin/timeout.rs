use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    guild::Permissions,
    util::Timestamp,
};
use std::{ error::Error, time::SystemTime };

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::DiscordClient,
        messages::{ DiscordEmbed, DiscordEmbedField },
    },
    utilities::utils::ColorResolvables,
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
        client: &DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ParsedArg::Users(users)) = command_args.first() {
            if let Some(ParsedArg::Number(duration)) = command_args.get(1) {
                if let Some(guild_id) = msg.guild_id {
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

                        match
                            client.http
                                .update_guild_member(guild_id, user.id)
                                .communication_disabled_until(Some(timestamp))?.await
                        {
                            Ok(_) => {
                                let message = client.get_locale_string(
                                    &config.locale,
                                    "command-timeout-success",
                                    Some(&args)
                                );
                                let _ = client.send_message(
                                    msg.channel_id,
                                    crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
                                        vec![DiscordEmbed {
                                            description: Some(message),
                                            color: Some(ColorResolvables::Green.as_u32()),
                                            ..Default::default()
                                        }]
                                    )
                                ).await;
                            }
                            Err(e) => {
                                let message = client.get_locale_string(
                                    &config.locale,
                                    "command-timeout-fail",
                                    Some(&args)
                                );
                                let _ = client.send_message(
                                    msg.channel_id,
                                    crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
                                        vec![DiscordEmbed {
                                            description: Some(message),
                                            fields: Some(
                                                vec![DiscordEmbedField {
                                                    name: "Error".to_string(),
                                                    value: format!("{}", e),
                                                    inline: false,
                                                }]
                                            ),
                                            color: Some(ColorResolvables::Red.as_u32()),
                                            ..Default::default()
                                        }]
                                    )
                                ).await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
