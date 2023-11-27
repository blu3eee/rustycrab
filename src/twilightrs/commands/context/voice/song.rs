use std::error::Error;

use async_trait::async_trait;
use twilight_model::{ gateway::payload::incoming::MessageCreate, channel::message::Embed };

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::DiscordClient,
        messages::{ DiscordEmbedField, DiscordEmbed },
    },
    utilities::{ format_duration, utils::ColorResolvables },
    cdn_avatar,
};

pub struct CurrentSongCommand {}

#[async_trait]
impl ContextCommand for CurrentSongCommand {
    fn name(&self) -> &'static str {
        "song"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["currentsong", "track", "playing"]
    }

    async fn run(
        &self,
        client: DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("Command not used in a guild")?;

        if !client.is_user_in_same_channel_as_bot(guild_id, msg.author.id).await? {
            client.http
                .create_message(msg.channel_id)
                .content(
                    "You need to be in the same voice channel as the bot to use this command"
                )?.await?;
            return Ok(());
        }

        if let Some((metadata, requested_by)) = client.voice_manager.get_current_song(guild_id) {
            client.http.create_message(msg.channel_id).embeds(
                &vec![
                    Embed::from(DiscordEmbed {
                        author_name: Some("Now playing".to_string()),
                        author_icon_url: Some(client.voice_manager.spinning_disk.clone()),
                        title: Some(
                            metadata.title.as_ref().unwrap_or(&"<UNKNOWN>".to_string()).to_string()
                        ),
                        url: metadata.source_url.map(|url| url),
                        thumbnail: if let Some(url) = metadata.thumbnail {
                            Some(url.to_string())
                        } else {
                            None
                        },
                        fields: Some(
                            vec![DiscordEmbedField {
                                name: format!("Duration"),
                                value: if let Some(duration) = metadata.duration.as_ref() {
                                    format_duration(duration)
                                } else {
                                    format!("<Unknown duration>")
                                },
                                inline: true,
                            }]
                        ),
                        footer_text: Some(format!("Requested by @{}", requested_by.name)),
                        footer_icon_url: requested_by.avatar.map(|hash|
                            cdn_avatar!(requested_by.id, hash)
                        ),
                        color: Some(ColorResolvables::Green.as_u32()),
                        ..Default::default()
                    })
                ]
            )?.await?;
        }

        Ok(())
    }
}
