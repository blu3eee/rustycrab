use self::{
    ping::PingCommand,
    avatar::AvatarCommand,
    math::MathCommand,
    help::HelpCommand,
    banner::BannerCommand,
    snipe::SnipeCommand,
    afk::AfkCommand,
};

use super::{ ContextCommandCategory, ContextCommand };

mod ping;
mod avatar;
mod math;
mod help;
mod banner;
mod snipe;
mod afk;

pub struct GeneralCommands;

impl ContextCommandCategory for GeneralCommands {
    fn name(&self) -> &'static str {
        "General"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([
            Box::new(PingCommand) as Box<dyn ContextCommand>,
            Box::new(AvatarCommand) as Box<dyn ContextCommand>,
            Box::new(MathCommand) as Box<dyn ContextCommand>,
            Box::new(HelpCommand) as Box<dyn ContextCommand>,
            Box::new(BannerCommand) as Box<dyn ContextCommand>,
            Box::new(SnipeCommand) as Box<dyn ContextCommand>,
            Box::new(AfkCommand) as Box<dyn ContextCommand>,
        ])
    }
}
