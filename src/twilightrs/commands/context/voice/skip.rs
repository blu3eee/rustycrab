use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    utilities::utils::ColorResolvables,
};
pub struct SkipCurrentTrackCommand {}

#[async_trait]
impl ContextCommand for SkipCurrentTrackCommand {
    fn name(&self) -> &'static str {
        "skip"
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

        // Scope to limit the lock guard
        let trackqueue = {
            let store = client.trackqueues.read().unwrap();
            store.get(&guild_id).cloned()
        };

        if let Some(trackqueue) = trackqueue {
            if let Some(_) = trackqueue.current() {
                match trackqueue.skip() {
                    Ok(_) => {}
                    Err(_) => {
                        if let Ok(_) = trackqueue.skip() {
                        } else {
                            client.reply_message(
                                msg.channel_id,
                                msg.id,
                                MessageContent::DiscordEmbeds(
                                    vec![DiscordEmbed {
                                        description: Some(format!("Failed to skip current track")),
                                        color: Some(ColorResolvables::Red.as_u32()),
                                        ..Default::default()
                                    }]
                                )
                            ).await?;
                        }
                    }
                }
                match trackqueue.current() {
                    Some(_) => {}
                    None => {
                        client.reply_message(
                            msg.channel_id,
                            msg.id,
                            MessageContent::DiscordEmbeds(
                                vec![DiscordEmbed {
                                    description: Some(format!("No more track to play..")),
                                    color: Some(ColorResolvables::Red.as_u32()),
                                    ..Default::default()
                                }]
                            )
                        ).await?;
                    }
                }

                return Ok(());
            }
        }
        client.http.create_message(msg.channel_id).content("No track is currently playing")?.await?;
        Ok(())
    }
}
