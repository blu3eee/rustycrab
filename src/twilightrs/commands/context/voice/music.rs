use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::DiscordClient,
        messages::DiscordEmbed,
    },
    cdn_avatar,
    utilities::utils::ColorResolvables,
};

use super::{
    play::PlayCommand,
    pause::PauseMusicCommand,
    resume::ResumeMusicCommand,
    stop::StopMusicCommand,
    skip::SkipCurrentTrackCommand,
    queue::QueueCommand,
    song::CurrentSongCommand,
    skip_to::SkipToTrackCommand,
    loop_music::LoopMusicCommand,
    unloop_music::UnloopMusicCommand,
};

pub struct MusicHelpCommand {}

#[async_trait]
impl ContextCommand for MusicHelpCommand {
    fn name(&self) -> &'static str {
        "music"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["music help"]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let _ = msg.guild_id.ok_or("Command not used in a guild")?;
        let bot = client.get_bot().await?;
        let music_commands: Vec<Box<dyn ContextCommand>> = Vec::from([
            Box::new(PlayCommand {}) as Box<dyn ContextCommand>,
            Box::new(PauseMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(ResumeMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(StopMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(QueueCommand {}) as Box<dyn ContextCommand>,
            Box::new(SkipCurrentTrackCommand {}) as Box<dyn ContextCommand>,
            Box::new(SkipToTrackCommand {}) as Box<dyn ContextCommand>,
            Box::new(CurrentSongCommand {}) as Box<dyn ContextCommand>,
            Box::new(LoopMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(UnloopMusicCommand {}) as Box<dyn ContextCommand>,
        ]);

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
            crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    author_name: Some(format!("Music commands - Prefix: {}", config.prefix)),
                    author_icon_url: Some(
                        "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
                    ),
                    thumbnail: bot.avatar.map(|avatar_hash| cdn_avatar!(bot.id, avatar_hash)),
                    description: Some(
                        format!("This feature only accepts Youtube URLs at the moment. Search results will get the first video from youtube search and add it to the queue.\n```fix\n{}```", description)
                    ),
                    color: Some(ColorResolvables::Blue.as_u32()),
                    ..Default::default()
                }]
            )
        ).await;

        Ok(())
    }
}
