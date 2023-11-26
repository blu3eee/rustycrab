use std::error::Error;

use async_trait::async_trait;
use songbird::input::{ YoutubeDl, Compose };
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    utilities::utils::ColorResolvables,
};
pub struct QueueCommand {}

#[async_trait]
impl ContextCommand for QueueCommand {
    fn name(&self) -> &'static str {
        "queue"
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

        let queued_urls = {
            let mut waiting_urls = client.waiting_track_urls.write().unwrap();
            waiting_urls.entry(guild_id).or_default().clone()
        };
        if queued_urls.len() == 0 {
            let _ = client.reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::Text(format!("Queue is empty!"))
            ).await;
            return Ok(());
        }
        let mut embed = DiscordEmbed {
            author_name: Some(format!("Queue: {} songs", queued_urls.len())),
            author_icon_url: Some(
                "https://cdn.darrennathanael.com/icons/spinning_disk.gif".to_string()
            ),
            color: Some(ColorResolvables::Blue.as_u32()),
            ..Default::default()
        };

        let sent_msg = client
            .reply_message(
                msg.channel_id,
                msg.id,
                MessageContent::DiscordEmbeds(vec![embed.clone()])
            ).await?
            .model().await?;

        let mut queued_songs: Vec<String> = Vec::new();
        for url in queued_urls[..(10).min(queued_songs.len())].to_vec() {
            let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());
            match source.aux_metadata().await {
                Ok(metadata) => {
                    queued_songs.push(
                        format!(
                            "[{}]({})",
                            metadata.title.unwrap_or("Unknown title".to_string()),
                            url
                        )
                    );
                }
                Err(_) => {
                    queued_songs.push(format!("[Unknown track]({})", url));
                }
            }
        }

        let pages = ((queued_urls.len() as f64) / (10 as f64)).ceil() as usize;

        let list = queued_songs[..(10).min(queued_songs.len())]
            .to_vec()
            .iter()
            .enumerate()
            .map(|(i, e)| format!("{}. {}", i, e))
            .collect::<Vec<String>>()
            .join("\n");

        embed.description = Some(list);

        let result = client.edit_message(
            sent_msg.channel_id,
            sent_msg.id,
            MessageContent::DiscordEmbeds(vec![embed.clone()])
        ).await;

        println!("result editing: {:?}", result);

        Ok(())
    }
}
