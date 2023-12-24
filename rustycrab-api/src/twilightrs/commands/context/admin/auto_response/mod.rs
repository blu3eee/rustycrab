mod utils;
mod add;
mod delete;
mod list;
mod image;
mod thumbnail;
mod color;
mod content;
mod text_message;

use add::AddAutoResponseCommand;
use delete::DeleteAutoResponseCommand;
use fluent_bundle::FluentArgs;
use list::ListAutoResponseCommand;
use image::ImageUpdateAutoResCommand;
use rustycrab_model::color::ColorResolvables;
use thumbnail::ThumbnailUpdateAutoResCommand;
use color::ColorUpdateAutoResCommand;
use content::ContentUpdateAutoResCommand;
use text_message::MessageUpdateAutoResCommand;

use async_trait::async_trait;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::{
    twilightrs::{
        commands::context::{ ContextCommand, ParsedArg, context_command::GuildConfigModel },
        discord_client::DiscordClient,
        messages::DiscordEmbed,
    },
    cdn_avatar,
};
pub struct AutoResCommand;

#[async_trait]
impl ContextCommand for AutoResCommand {
    fn name(&self) -> &'static str {
        "autores"
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::ADMINISTRATOR]
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["ar", "autoresponse"]
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        vec![
            Box::new(AddAutoResponseCommand {}) as Box<dyn ContextCommand>,
            Box::new(DeleteAutoResponseCommand {}) as Box<dyn ContextCommand>,
            Box::new(ListAutoResponseCommand {}) as Box<dyn ContextCommand>,
            Box::new(ImageUpdateAutoResCommand {}) as Box<dyn ContextCommand>,
            Box::new(ThumbnailUpdateAutoResCommand {}) as Box<dyn ContextCommand>,
            Box::new(ColorUpdateAutoResCommand {}) as Box<dyn ContextCommand>,
            Box::new(ContentUpdateAutoResCommand {}) as Box<dyn ContextCommand>,
            Box::new(MessageUpdateAutoResCommand {}) as Box<dyn ContextCommand>
        ]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let _ = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let _ = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        let bot = client.get_bot().await?;
        let music_commands: Vec<Box<dyn ContextCommand>> = self.subcommands();

        let description = music_commands
            .iter()
            .map(|command| {
                let description = command.description(&config.locale);

                let (usage, _, _, _) = command.get_help(&config.locale, format!(""), &vec![]);

                format!(
                    "{}{}",
                    usage,
                    description.map_or_else(
                        || format!(""),
                        |desc| format!(": {}", desc)
                    )
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let _ = client.reply_message(
            msg.channel_id,
            msg.id,
            crate::twilightrs::discord_client::MessageContent::TextAndDiscordEmbeds(
                format!("```fix\n{}```", description),
                vec![DiscordEmbed {
                    author_name: Some(
                        format!("Auto-Response commands - Prefix: {}", config.prefix)
                    ),
                    thumbnail: bot.avatar.map(|avatar_hash| cdn_avatar!(bot.id, avatar_hash)),
                    color: Some(ColorResolvables::Blue.as_u32()),
                    footer_text: Some(
                        client.get_locale_string(
                            &config.locale,
                            "requested-user",
                            Some(
                                &FluentArgs::from_iter(vec![("username", msg.author.name.clone())])
                            )
                        )
                    ),
                    footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                    ..Default::default()
                }]
            )
        ).await;

        Ok(())
    }
}
