use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        commands::context::ContextCommand,
        client::{ DiscordClient, MessageContent, DiscordEmbed },
        utils::greedy::greedy_user,
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

    async fn run(
        &self,
        client: &DiscordClient,
        _: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        args: &[&str]
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let bot = client.http.current_user().await?.model().await?;

        println!("bot {:?}", bot);

        let user = if let (Some(u), _) = greedy_user(&client.http, args).await {
            u.clone()
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
