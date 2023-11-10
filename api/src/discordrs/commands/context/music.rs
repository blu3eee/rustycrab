use async_trait::async_trait;
// commands/context/music.rs
use discord::{ model::{ Message, ServerId, ChannelId }, voice::VoiceConnection };

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    utilities::warn,
    discordrs::client::DiscordClient,
};

use super::CommandHandler;

// Example implementation for a MusicCommand
pub struct MusicCommand;

#[async_trait]
impl CommandHandler for MusicCommand {
    async fn handle_command(
        &self,
        client: &mut DiscordClient,
        _: &GuildConfig,
        message: &Message,
        args: &[&str]
    ) {
        if args.len() > 0 {
            let argument: &str = args[0];
            let vchan: Option<(Option<ServerId>, ChannelId)> = client.state.find_voice_user(
                message.author.id
            );
            if argument.eq_ignore_ascii_case("stop") {
                vchan.map(|(sid, _)| client.connection.voice(sid).stop());
            } else if argument.eq_ignore_ascii_case("quit") {
                vchan.map(|(sid, _)| client.connection.drop_voice(sid));
            } else {
                let output: String = if let Some((server_id, channel_id)) = vchan {
                    match discord::voice::open_ytdl_stream(argument) {
                        Ok(stream) => {
                            println!("[DJ] Downloaded {}", argument);
                            let voice: &mut VoiceConnection = client.connection.voice(server_id);
                            voice.connect(channel_id);
                            voice.set_deaf(true);
                            voice.play(stream);
                            String::new()
                        }
                        Err(error) => format!("Error: {}", error),
                    }
                } else {
                    "You must be in a voice channel to DJ".to_owned()
                };
                if !output.is_empty() {
                    warn(client.discord.send_message(message.channel_id, &output, "", false));
                }
            }
        }
    }
}
