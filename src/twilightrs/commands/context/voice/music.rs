use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
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
        let _ = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
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
            crate::twilightrs::discord_client::MessageContent::TextAndDiscordEmbeds(
                format!("```fix\n{}```", description),
                vec![DiscordEmbed {
                    author_name: Some(format!("Music commands - Prefix: {}", config.prefix)),
                    author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
                    thumbnail: bot.avatar.map(|avatar_hash| cdn_avatar!(bot.id, avatar_hash)),
                    description: Some(
                        format!(
                            "{}\n\n**Disclaimer:**\nThis is a project developed for educational purposes only. This bot is not affiliated, associated, authorized, endorsed by, or in any way officially connected with YouTube, SoundCloud, or Discord, or any of their subsidiaries or affiliates.\n\nThis bot is intended to be used in compliance with all applicable copyright laws and the terms of service of the platforms it interacts with. As the developer of this bot, I assume no liability for any misuse of this bot by others and encourage all users to ensure they adhere to the legal terms of the platforms involved.\n\nUsers of this bot should be aware that they are responsible for ensuring their use of the bot is compliant with YouTube, SoundCloud, and Discord's terms of service, as well as applicable laws. The developer of this bot does not endorse or condone any unauthorized use of copyrighted content.",
                            client.get_locale_string(&config.locale, "music-note", None)
                        )
                    ),
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
