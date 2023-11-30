// mod tts;
mod music;
mod join;
mod leave;
mod play;
mod pause;
mod resume;
mod stop;
mod skip;
mod queue;
mod song;
mod skip_to;
mod loop_music;
mod loopq_music;
mod unloop_music;

use self::{
    play::PlayCommand,
    join::JoinCommand,
    leave::LeaveChannelCommand,
    pause::PauseMusicCommand,
    resume::ResumeMusicCommand,
    stop::StopMusicCommand,
    skip::SkipCurrentTrackCommand,
    queue::QueueCommand,
    song::CurrentSongCommand,
    skip_to::SkipToTrackCommand,
    loop_music::LoopMusicCommand,
    unloop_music::UnloopMusicCommand,
    music::MusicHelpCommand,
    loopq_music::LoopQueueMusicCommand,
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
            Box::new(MusicHelpCommand {}) as Box<dyn ContextCommand>,
            Box::new(PlayCommand {}) as Box<dyn ContextCommand>,
            Box::new(PauseMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(ResumeMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(StopMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(SkipCurrentTrackCommand {}) as Box<dyn ContextCommand>,
            Box::new(QueueCommand {}) as Box<dyn ContextCommand>,
            Box::new(CurrentSongCommand {}) as Box<dyn ContextCommand>,
            Box::new(SkipToTrackCommand {}) as Box<dyn ContextCommand>,
            Box::new(LoopMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(LoopQueueMusicCommand {}) as Box<dyn ContextCommand>,
            Box::new(UnloopMusicCommand {}) as Box<dyn ContextCommand>,
        ])
    }
}

pub async fn join_voice_channel() {}
