use super::commands::context::context_command_dispatcher::ContextCommandDispatcher;

pub struct ClientDispatchers {
    pub context_commands: ContextCommandDispatcher,
}

impl ClientDispatchers {
    pub fn new() -> Self {
        Self {
            context_commands: ContextCommandDispatcher::new(),
        }
    }
}
