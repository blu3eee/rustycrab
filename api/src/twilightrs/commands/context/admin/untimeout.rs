use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::{
    database::bot_guild_configurations::Model as GuildConfigModel,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::DiscordClient,
        messages::{ DiscordEmbed, DiscordEmbedField },
    },
    utilities::utils::ColorResolvables,
};

pub struct UntimeoutMemberCommand;

#[async_trait]
impl ContextCommand for UntimeoutMemberCommand {
    fn name(&self) -> &'static str {
        "untimeout"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("users", ArgType::Users, false)]
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["unmute"]
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
            if let Some(guild_id) = msg.guild_id {
                for user in users {
                    let mut args = FluentArgs::new();
                    args.set("user", format!("<@{}>", user.id.to_string()));

                    if client.http.guild_member(guild_id, user.id).await?.model().await?.mute {
                        match
                            client.http
                                .update_guild_member(guild_id, user.id)
                                .communication_disabled_until(None)?.await
                        {
                            Ok(_) => {
                                let message = client.get_locale_string(
                                    &config.locale,
                                    "command-untimeout-success",
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
                                    "command-untimeout-fail",
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
                    } else {
                        let message = client.get_locale_string(
                            &config.locale,
                            "command-untimeout-notfound",
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
                }
            }
        }

        Ok(())
    }
}
