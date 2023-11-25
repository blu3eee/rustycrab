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
pub struct BanMemberCommand;

#[async_trait]
impl ContextCommand for BanMemberCommand {
    fn name(&self) -> &'static str {
        "ban"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![
            ArgSpec::new("users", ArgType::Users, false),
            ArgSpec::new("reason", ArgType::Text, true)
        ]
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::BAN_MEMBERS]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        if let Some(ParsedArg::Users(users)) = command_args.first() {
            if let Some(guild_id) = msg.guild_id {
                for user in users {
                    let mut args = FluentArgs::new();
                    args.set("user", format!("<@{}>", user.id.to_string()));
                    // Ban each user
                    if
                        client.cache
                            .permissions()
                            .in_channel(user.id, msg.channel_id)?
                            .contains(Permissions::ADMINISTRATOR)
                    {
                        let message = client.get_locale_string(
                            &config.locale,
                            "command-ban-admin",
                            Some(&args)
                        );
                        let _ = client.send_message(
                            msg.channel_id,
                            crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
                                vec![DiscordEmbed {
                                    description: Some(message),
                                    color: Some(ColorResolvables::Red.as_u32()),
                                    ..Default::default()
                                }]
                            )
                        ).await;
                        continue;
                    }

                    // Ban user and handle response
                    match client.http.create_ban(guild_id, user.id).await {
                        Ok(_) => {
                            let message = client.get_locale_string(
                                &config.locale,
                                "command-ban-success",
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
                                "command-ban-fail",
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

        Ok(())
    }
}
