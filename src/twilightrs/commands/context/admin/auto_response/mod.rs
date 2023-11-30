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
use list::ListAutoResponseCommand;
use image::ImageUpdateAutoResCommand;
use thumbnail::ThumbnailUpdateAutoResCommand;
use color::ColorUpdateAutoResCommand;
use content::ContentUpdateAutoResCommand;
use text_message::MessageUpdateAutoResCommand;

use async_trait::async_trait;
use twilight_model::{ gateway::payload::incoming::MessageCreate, guild::Permissions };
use std::error::Error;

use crate::twilightrs::{
    commands::context::{ ContextCommand, ParsedArg, context_command::GuildConfigModel },
    discord_client::DiscordClient,
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

        Ok(())
    }
}
