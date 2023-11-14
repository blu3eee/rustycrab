use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::{ DiscordClient, MessageContent },
        embeds::DiscordEmbed,
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
        vec![ArgSpec::new(ArgType::User, true)] // User argument is optional
    }

    async fn run(
        &self,
        client: &DiscordClient,
        _: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let bot = client.get_bot().await?;

        let user = if let Some(ParsedArg::User(user)) = command_args.get(0) {
            user.clone()
        } else {
            msg.author.clone()
        };

        let sent_msg = client
            .send_message(
                msg.channel_id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        description: Some(format!("Getting <@{}> avatar..", user.id)),
                        ..Default::default()
                    }]
                )
            ).await?
            .model().await?;

        if let Some(avatar_hash) = user.avatar {
            let _ = client.edit_message(
                msg.channel_id,
                sent_msg.id,
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
                            Some(format!("Requested by: @{}", &msg.author.name))
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
