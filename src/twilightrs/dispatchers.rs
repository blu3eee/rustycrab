use super::commands::{
    context::context_command_dispatcher::ContextCommandDispatcher,
    slash::slash_command_dispatcher::SlashCommandDispatcher,
};

pub struct ClientDispatchers {
    pub context_commands: ContextCommandDispatcher,
    pub slash_commands: SlashCommandDispatcher,
}

impl ClientDispatchers {
    pub fn new() -> Self {
        Self {
            context_commands: ContextCommandDispatcher::new(),
            slash_commands: SlashCommandDispatcher::new(),
        }
    }
}
