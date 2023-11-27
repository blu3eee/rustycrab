use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::{ DiscordClient, MessageContent },
    bot::voice_manager::PlayerLoopState,
};
pub struct UnloopMusicCommand {}

#[async_trait]
impl ContextCommand for UnloopMusicCommand {
    fn name(&self) -> &'static str {
        "unloop"
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

        match client.voice_manager.get_loop_state(guild_id) {
            PlayerLoopState::LoopCurrentTrack => {
                if let Some(track_handle) = client.voice_manager.get_play_queue(guild_id).current() {
                    track_handle.disable_loop()?;
                }
            }
            _ => {}
        }

        client.voice_manager.set_loop_state(guild_id, PlayerLoopState::NoLoop);
        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::Text("Looping disabled".to_string())
        ).await?;

        Ok(())
    }
}
