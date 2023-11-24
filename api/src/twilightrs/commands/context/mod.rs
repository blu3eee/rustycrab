pub mod admin;
pub mod general;
pub mod voice;

pub mod context_command_dispatcher;
pub mod context_command;

use twilight_model::{ user::User, channel::Channel };

use self::context_command::ContextCommand;

/// Argument types for command parsing
// #[derive(Debug)]
pub enum ArgType {
    Arg,
    Args,
    Text,
    Number,
    User,
    Channel,
    Users, // List of user IDs
    Channels, // List of channel IDs
}

/// Specification for command arguments
pub struct ArgSpec {
    name: &'static str,
    arg_type: ArgType,
    optional: bool,
}

impl ArgSpec {
    /// Create a new argument specification
    pub fn new(name: &'static str, arg_type: ArgType, optional: bool) -> Self {
        ArgSpec { name, arg_type, optional }
    }

    pub fn to_string(&self) -> String {
        if self.optional { format!("[{}]", self.name) } else { format!("<{}>", self.name) }
    }
}

/// Parsed command argument types
pub enum ParsedArg {
    None,
    Word(String),
    Words(Vec<String>),
    Text(String),
    Number(i64),
    User(User),
    Users(Vec<User>),
    Channel(Channel),
    Channels(Vec<Channel>),
}

/// Trait defining a context command category
pub trait ContextCommandCategory {
    fn name(&self) -> &'static str;
    fn collect_commands(&self) -> Vec<Box<dyn ContextCommand>>;
}

/// Handler for context commands
pub struct ContextCommandHandler {
    pub command_name: &'static str,
    pub category_name: &'static str,
    pub command: Box<dyn ContextCommand>,
}
