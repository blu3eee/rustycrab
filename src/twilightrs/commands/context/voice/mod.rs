// mod tts;
mod join;
mod leave;
mod play;
mod pause;
mod resume;
mod stop;
mod skip;

mod song_bird_event_handler;

use self::{
    play::PlayCommand,
    join::JoinCommand,
    leave::LeaveChannelCommand,
    pause::PauseMusicCommand,
    resume::ResumeMusicCommand,
    stop::StopMusicCommand,
    skip::SkipCurrentTrackCommand,
};

use super::{ ContextCommandCategory, context_command::ContextCommand };

pub struct VoiceCommands;

impl ContextCommandCategory for VoiceCommands {
    fn name(&self) -> &'static str {
        "Voice & Music"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([
            Box::new(JoinCommand {}) as Box<dyn ContextCommand>,
            Box::new(LeaveChannelCommand {}) as Box<dyn ContextCommand>,
            Box::new(PlayCommand {}) as Box<dyn ContextCommand>,
            Box::new(PauseMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(ResumeMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(StopMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(SkipCurrentTrackCommand {}) as Box<dyn ContextCommand>,
        ])
    }
}

pub async fn join_voice_channel() {}
