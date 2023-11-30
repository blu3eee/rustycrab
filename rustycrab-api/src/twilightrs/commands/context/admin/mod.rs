mod prefix;
mod language;
mod purge;
mod ban;
mod unban;
mod kick;
mod timeout;
mod untimeout;
mod set_role;
mod auto_response;

use self::{
    prefix::ChangePrefixCommand,
    language::ChangeLanguageCommand,
    purge::PurgeCommand,
    ban::BanMemberCommand,
    unban::UnbanMemberCommand,
    kick::KickMemberCommand,
    timeout::TimeoutMemberCommand,
    untimeout::UntimeoutMemberCommand,
    set_role::RoleCommand,
    auto_response::AutoResCommand,
};
use super::{ ContextCommandCategory, context_command::ContextCommand };

pub struct AdminCommands;

impl ContextCommandCategory for AdminCommands {
    fn name(&self) -> &'static str {
        "Administrator"
    }

    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::from([
            Box::new(ChangePrefixCommand) as Box<dyn ContextCommand>,
            Box::new(ChangeLanguageCommand) as Box<dyn ContextCommand>,
            Box::new(PurgeCommand) as Box<dyn ContextCommand>,
            Box::new(BanMemberCommand) as Box<dyn ContextCommand>,
            Box::new(UnbanMemberCommand) as Box<dyn ContextCommand>,
            Box::new(KickMemberCommand) as Box<dyn ContextCommand>,
            Box::new(TimeoutMemberCommand) as Box<dyn ContextCommand>,
            Box::new(UntimeoutMemberCommand) as Box<dyn ContextCommand>,
            Box::new(RoleCommand) as Box<dyn ContextCommand>,
            Box::new(AutoResCommand) as Box<dyn ContextCommand>,
        ])
    }
}
