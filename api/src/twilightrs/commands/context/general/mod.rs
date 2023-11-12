use self::ping::PingCommand;

use super::{ ContextCommandCategory, ContextCommand };

pub mod ping;

pub struct GeneralCommands;

impl ContextCommandCategory for GeneralCommands {
    fn name(&self) -> &'static str {
        "General"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([Box::new(PingCommand {}) as Box<dyn ContextCommand>])
    }
}
