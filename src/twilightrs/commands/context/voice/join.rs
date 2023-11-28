use std::error::Error;

use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{
            context_command::{ ContextCommand, GuildConfigModel },
            ParsedArg,
            ArgSpec,
            ArgType,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::DiscordEmbed,
    },
    utilities::utils::ColorResolvables,
    cdn_avatar,
};

pub struct JoinCommand {}

#[async_trait]
impl ContextCommand for JoinCommand {
    fn name(&self) -> &'static str {
        "join"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("voice channel", ArgType::Channel, true)]
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
        let mut args = FluentArgs::new();

        let (key, color) = match client.cache.voice_state(msg.author.id, guild_id) {
            Some(state) => {
                // Ensure the user is in a voice channel
                let channel_id = state.channel_id();
                args.set("channel", format!("<#{}>", channel_id));

                let join_result = client.voice_music_manager.songbird.join(
                    guild_id,
                    channel_id
                ).await;

                match join_result {
                    Ok(_call_lock) => {
                        // Successfully joined the channel
                        ("command-join-joined", ColorResolvables::Green)
                    }
                    Err(e) => {
                        // Failed to join the channel
                        args.set("err", e.to_string());
                        ("command-join-failed", ColorResolvables::Red)
                    }
                }
            }
            None => {
                // Notify user they need to be in a voice channel

                ("command-join-nochannel", ColorResolvables::Red)
            }
        };

        let content = client.get_locale_string(&config.locale, key, Some(&args));
        // Notify user about the result
        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    description: Some(content),
                    color: Some(color.as_u32()),
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
        ).await?;

        Ok(())
    }
}

impl JoinCommand {}
