use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::{ DiscordEmbed, DiscordEmbedField },
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
};
pub struct SkipCurrentTrackCommand {}

#[async_trait]
impl ContextCommand for SkipCurrentTrackCommand {
    fn name(&self) -> &'static str {
        "skip"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["next"]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;

        let _ = client.fetch_call_lock(guild_id, Some(&config.locale)).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id, Some(&config.locale)).await?;

        let handle = client.fetch_trackhandle(guild_id, Some(&config.locale)).await?;

        let skipped_track = client.voice_music_manager.get_current_song(guild_id);

        let mut args = FluentArgs::new();
        args.set("username", msg.author.name.clone());

        handle
            .stop()
            .map_err(|_|
                client.get_locale_string(&config.locale, "command-skip-failed", Some(&args))
            );
        // let embed = if let Ok(_) = handle.stop() {

        // } else {
        //     DiscordEmbed {
        //         description: Some(
        //             client.get_locale_string(&config.locale, "command-skip-failed", Some(&args))
        //         ),
        //         color: Some(ColorResolvables::Red.as_u32()),
        //         ..Default::default()
        //     }
        // };

        let embed = if let Some((metadata, requested_user)) = skipped_track {
            let title = metadata.title.unwrap_or("Unknown".to_string());
            let url = metadata.source_url.unwrap_or("Unknown".to_string());
            args.set("title", format!("[{}]({})", title, url));
            DiscordEmbed {
                description: Some(
                    client.get_locale_string(&config.locale, "command-skip-skipped", Some(&args))
                ),
                footer_text: Some(
                    client.get_locale_string(&config.locale, "command-skip-author", Some(&args))
                ),
                footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                fields: Some(
                    vec![DiscordEmbedField {
                        name: client.get_locale_string(
                            &config.locale,
                            "command-skip-requested-by",
                            Some(&args)
                        ),
                        value: format!("<@{}>", requested_user.id),
                        inline: false,
                    }]
                ),
                color: Some(ColorResolvables::Yellow.as_u32()),
                ..Default::default()
            }
        } else {
            DiscordEmbed {
                description: Some(
                    client.get_locale_string(
                        &config.locale,
                        "command-skip-no-metadata",
                        Some(&args)
                    )
                ),
                footer_text: Some(
                    client.get_locale_string(&config.locale, "command-skip-author", Some(&args))
                ),
                footer_icon_url: msg.author.avatar.map(|hash| cdn_avatar!(msg.author.id, hash)),
                color: Some(ColorResolvables::Yellow.as_u32()),
                ..Default::default()
            }
        };

        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(vec![embed])
        ).await?;

        Ok(())
    }
}
