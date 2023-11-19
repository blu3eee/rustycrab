use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, ArgSpec, ArgType },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    cdn_avatar,
};

pub struct BannerCommand;

#[async_trait]
impl ContextCommand for BannerCommand {
    fn name(&self) -> &'static str {
        "banner"
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

        match client.get_user_banner_url(user.id).await {
            Ok(Some(banner_url)) => {
                println!("{}", banner_url);
                let _ = client.reply_message(
                    msg.channel_id,
                    msg.id,
                    MessageContent::DiscordEmbeds(
                        vec![DiscordEmbed {
                            title: Some(format!("{}'s banner", &user.name)),
                            author_name: Some(format!("@{}", &bot.name)),
                            author_icon_url: if let Some(bot_avatar) = &bot.avatar {
                                Some(cdn_avatar!(bot.id, bot_avatar.to_string()))
                            } else {
                                None
                            },
                            image: Some(banner_url),
                            footer_text: if msg.author.id == user.id {
                                None
                            } else {
                                Some(format!("Requested by: @{}", &msg.author.name))
                            },
                            footer_icon_url: if msg.author.id != user.id {
                                msg.author.avatar
                                    .as_ref()
                                    .map(|hash| cdn_avatar!(msg.author.id, hash))
                            } else {
                                None
                            },
                            timestamp: Some(true),
                            ..Default::default()
                        }]
                    )
                ).await;
            }
            Ok(None) => {
                client.send_message(
                    msg.channel_id,
                    MessageContent::Text(format!("No banner found for user {}", msg.author.name))
                ).await?;
            }
            Err(_) => {
                client.send_message(
                    msg.channel_id,
                    MessageContent::Text("Error fetching banner".to_string())
                ).await?;
            }
        }

        Ok(())
    }
}
