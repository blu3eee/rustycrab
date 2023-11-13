use self::{ ping::PingCommand, avatar::AvatarCommand };

use super::{ ContextCommandCategory, ContextCommand };

mod ping;
mod avatar;
mod help;
pub struct GeneralCommands;

impl ContextCommandCategory for GeneralCommands {
    fn name(&self) -> &'static str {
        "General"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([
            Box::new(PingCommand {}) as Box<dyn ContextCommand>,
            Box::new(AvatarCommand {}) as Box<dyn ContextCommand>,
        ])
    }
}
