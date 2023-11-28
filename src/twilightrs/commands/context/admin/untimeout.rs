use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
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
        utils::send_command_response,
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
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        if let Some(ParsedArg::Users(users)) = command_args.first() {
            for user in users {
                let mut args = FluentArgs::new();
                args.set("user", format!("<@{}>", user.id.to_string()));

                let (key, color) = if
                    client.http.guild_member(guild_id, user.id).await?.model().await?.mute
                {
                    match
                        client.http
                            .update_guild_member(guild_id, user.id)
                            .communication_disabled_until(None)?.await
                    {
                        Ok(_) => { ("command-untimeout-success", ColorResolvables::Green) }
                        Err(e) => {
                            args.set("err", format!("{}", e));
                            ("command-untimeout-fail", ColorResolvables::Red)
                        }
                    }
                } else {
                    ("command-untimeout-notfound", ColorResolvables::Yellow)
                };

                let _ = send_command_response(&client, &config, &msg, key, Some(args), color).await;
            }
        }

        Ok(())
    }
}
