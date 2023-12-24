use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use rustycrab_model::color::ColorResolvables;
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
    utils::send_command_response,
};

pub struct KickMemberCommand;

#[async_trait]
impl ContextCommand for KickMemberCommand {
    fn name(&self) -> &'static str {
        "kick"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![
            ArgSpec::new("users", ArgType::Users, false),
            ArgSpec::new("reason", ArgType::Text, true)
        ]
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::KICK_MEMBERS]
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
            for user in users {
                let mut args = FluentArgs::new();
                args.set("user", format!("<@{}>", user.id.to_string()));

                let (key, color) = if
                    client.cache
                        .permissions()
                        .in_channel(user.id, msg.channel_id)?
                        .contains(Permissions::ADMINISTRATOR)
                {
                    ("command-kick-admin", ColorResolvables::Red)
                } else {
                    match client.http.remove_guild_member(guild_id, user.id).await {
                        Ok(_) => { ("command-kick-success", ColorResolvables::Green) }
                        Err(e) => {
                            args.set("err", format!("{}", e));
                            ("command-kick-failed", ColorResolvables::Red)
                        }
                    }
                };

                let _ = send_command_response(&client, &config, &msg, key, Some(args), color).await;
            }
        }

        Ok(())
    }
}
