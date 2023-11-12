use async_trait::async_trait;
use discord::model::Message;
use crate::{
    discordrs::{
        client::DiscordClient,
        commands::context::ContextCommand,
        utils::greedy::greedy_user,
        MessageContent,
        DiscordEmbed,
    },
    database::bot_guild_configurations::Model as GuildConfig,
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

    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        _: &GuildConfig,
        message: &Message,
        args: &[&str]
    ) {
        let user = if let (Some(u), _) = greedy_user(client, args) {
            u.clone()
        } else {
            message.author.clone()
        };

        if let Some(avatar_hash) = user.avatar {
            let _ = client.send_message(
                message.channel_id,
                MessageContent::Embed(DiscordEmbed {
                    title: Some(format!("@{}'s avatar", &user.name)),
                    author_name: Some(format!("@{}", &client.state.user().username)),
                    author_icon_url: if let Some(bot_avatar) = &client.state.user().avatar {
                        Some(cdn_avatar!(client.state.user().id, bot_avatar))
                    } else {
                        None
                    },
                    image: Some(cdn_avatar!(user.id, avatar_hash)),
                    footer_text: if message.author.id == user.id {
                        None
                    } else {
                        Some(format!("Requested by: @{}", &message.author.name))
                    },
                    footer_icon_url: if message.author.id != user.id {
                        message.author.avatar
                            .as_ref()
                            .map(|hash| cdn_avatar!(message.author.id, hash))
                    } else {
                        None
                    },
                    ..Default::default()
                })
            );
        } else {
            println!("cant find avatar_url")
        }
    }
}
