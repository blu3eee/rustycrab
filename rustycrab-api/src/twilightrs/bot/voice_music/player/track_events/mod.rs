use async_trait::async_trait;
use songbird::{ input::AuxMetadata, tracks::PlayMode, Event, EventContext, EventHandler };
use twilight_model::{ user::User, id::{ marker::{ ChannelMarker, GuildMarker }, Id } };

use crate::twilightrs::discord_client::DiscordClient;

use self::{
    play::handle_event_track_play,
    stop::handle_event_track_end,
    error::handle_event_track_error,
};

mod error;
mod play;
mod stop;

pub struct MusicEventHandler {
    pub client: DiscordClient,
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    pub url: String,
    pub metadata: AuxMetadata,
    pub requested_by: User,
}

#[async_trait]
impl EventHandler for MusicEventHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_states) => {
                for (track_state, _) in *track_states {
                    let locale = if
                        let Ok(config) = self.client.get_guild_config(&self.guild_id).await
                    {
                        config.locale.clone()
                    } else {
                        "en".to_string()
                    };

                    if track_state.playing == PlayMode::Play {
                        let _ = handle_event_track_play(&self, &locale).await;
                    } else if track_state.playing == PlayMode::End {
                        let _ = handle_event_track_end(&self, &locale).await;
                    } else if let PlayMode::Errored(_) = &track_state.playing {
                        let _ = handle_event_track_error(&self, &locale);
                    } else if track_state.playing == PlayMode::Stop {
                        println!("player stopped");
                    } else if track_state.playing == PlayMode::Pause {
                        println!("player paused");
                    }
                }
            }
            _ => {}
        }
        None
    }
}
