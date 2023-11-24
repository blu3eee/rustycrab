// mod tts;

use super::{ ContextCommandCategory, context_command::ContextCommand };

pub struct GeneralCommands;

impl ContextCommandCategory for GeneralCommands {
    fn name(&self) -> &'static str {
        "Voice"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([])
    }
}
