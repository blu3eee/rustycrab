pub mod general;
pub mod voice;
pub mod context_command_dispatcher;
pub mod context_command;
use std::fmt;

use twilight_model::{ user::User, channel::Channel };

use self::context_command::ContextCommand;

/// Argument types for command parsing
// #[derive(Debug)]
pub enum ArgType {
    Word,
    Words,
    Text,
    Number,
    User,
    Channel,
    Users, // List of user IDs
    Channels, // List of channel IDs
}

impl fmt::Debug for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgType::Word => write!(f, "argument"),
            ArgType::Words => write!(f, "arguments"),
            ArgType::Text => write!(f, "text"),
            ArgType::Number => write!(f, "number"),
            ArgType::User => write!(f, "@user/userid"),
            ArgType::Channel => write!(f, "#channel/channel id"),
            ArgType::Users => write!(f, "@users/userids"),
            ArgType::Channels => write!(f, "#channels/channel ids"),
        }
    }
}

/// Specification for command arguments
pub struct ArgSpec {
    arg_type: ArgType,
    optional: bool,
}

impl ArgSpec {
    /// Create a new argument specification
    pub fn new(arg_type: ArgType, optional: bool) -> Self {
        ArgSpec { arg_type, optional }
    }

    pub fn to_string(&self) -> String {
        if self.optional {
            format!("[{:?}]", self.arg_type)
        } else {
            format!("<{:?}>", self.arg_type)
        }
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
