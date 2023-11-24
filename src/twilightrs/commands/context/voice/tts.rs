use std::error::Error;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{
        context_command::{ ContextCommand, GuildConfigModel },
        ParsedArg,
        ArgSpec,
        ArgType,
    },
    discord_client::DiscordClient,
};

pub struct TtsCommand {}

#[async_trait]
impl ContextCommand for TtsCommand {
    fn name(&self) -> &'static str {
        "tts"
    }

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new(ArgType::Text, false)]
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::new()
    }

    async fn run(
        &self,
        client: &DiscordClient,
        _: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ParsedArg::Text(text)) = command_args.get(0) {
            // Check if the user is in a voice channel
            // let voice_channel_id = client.get_user_voice_channel(msg.author.id).await?;
            // if voice_channel_id.is_none() {
            //     // Reply with a message if the user is not in a voice channel
            //     client.reply_message(
            //         msg.channel_id,
            //         msg.id,
            //         "You need to be in a voice channel to use this command."
            //     ).await?;
            //     return Ok(());
            // }

            // // Call the TTS API to get an audio stream
            // let audio_stream = self.call_tts_api(text).await?;

            // // Join the voice channel and play the audio
            // self.play_audio_in_voice_channel(
            //     client,
            //     voice_channel_id.unwrap(),
            //     audio_stream
            // ).await?;
        } else {
            // Handle the case where no text is provided
            // client.reply_message(msg.channel_id, msg.id, "Please provide some text.").await?;
        }
        Ok(())
    }
}

impl TtsCommand {
    // async fn call_tts_api(&self, text: &str) -> Result<AudioStream, Box<dyn Error + Send + Sync>> {
    //     // Implement the logic to call a TTS API and get an audio stream
    //     // This could involve sending a request to Google Cloud Text-to-Speech or another service
    //     todo!()
    // }

    // async fn play_audio_in_voice_channel(
    //     &self,
    //     client: &DiscordClient,
    //     channel_id: u64,
    //     audio_stream: AudioStream
    // ) -> Result<(), Box<dyn Error + Send + Sync>> {
    //     // Use songbird to play the audio in the specified voice channel

    //     todo!()
    // }
}
