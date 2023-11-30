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
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    cdn_avatar,
};
pub struct AvatarCommand;

#[async_trait]
impl ContextCommand for AvatarCommand {
    fn name(&self) -> &'static str {
        "avatar"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["av"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("user", ArgType::User, true)] // User argument is optional
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let bot = client.get_bot().await?;

        let user = if let Some(ParsedArg::User(user)) = command_args.get(0) {
            user.clone()
        } else {
            msg.author.clone()
        };

        if let Some(avatar_hash) = user.avatar {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        title: Some(format!("{}'s avatar", &user.name)),
                        author_name: Some(format!("@{}", &bot.name)),
                        author_icon_url: if let Some(bot_avatar) = &bot.avatar {
                            Some(cdn_avatar!(bot.id, bot_avatar.to_string()))
                        } else {
                            None
                        },
                        image: Some(cdn_avatar!(user.id, avatar_hash)),
                        footer_text: if msg.author.id == user.id {
                            None
                        } else {
                            Some(
                                client.get_locale_string(
                                    &config.locale,
                                    "requested-user",
                                    Some(
                                        &FluentArgs::from_iter(
                                            vec![("username", msg.author.name.clone())]
                                        )
                                    )
                                )
                            )
                        },
                        footer_icon_url: if msg.author.id != user.id {
                            msg.author.avatar.as_ref().map(|hash| cdn_avatar!(msg.author.id, hash))
                        } else {
                            None
                        },
                        timestamp: Some(true),
                        ..Default::default()
                    }]
                )
            ).await;
        } else {
            println!("cant find avatar_url");
        }

        Ok(())
    }
}
