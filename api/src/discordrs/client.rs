use crate::{ utilities::app_error::AppError, BotId };
use discord::{ self, model::{ ChannelId, Message, MessageId }, Discord, Connection, State };
use sea_orm::DatabaseConnection;

use super::{
    MessageContent,
    utils::messages::{ send_message, edit_message },
    commands::context::dispatcher::ContextCommandDispatcher,
};

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

pub struct DiscordClient {
    pub db: DatabaseConnection,
    pub bot_id: BotId,
    pub discord: Discord,
    pub connection: Connection,
    pub state: State,
}

impl DiscordClient {
    // Add an associated function to create a new instance of DiscordClient
    pub fn new(
        db: DatabaseConnection,
        bot_id: BotId,
        discord: Discord,
        connection: Connection,
        state: State
    ) -> Self {
        Self {
            db,
            bot_id,
            discord,
            connection,
            state,
        }
    }

    pub fn send_message(
        &self,
        channel_id: ChannelId,
        message_content: MessageContent
    ) -> Result<Message, AppError> {
        send_message(self, channel_id, message_content)
    }

    pub async fn edit_message(
        &mut self,
        channel_id: ChannelId,
        message_id: MessageId,
        message_content: MessageContent
    ) -> Result<Message, AppError> {
        edit_message(self, channel_id, message_id, message_content)
    }

    pub async fn reply_message(&mut self, channel_id: ChannelId, message_content: MessageContent) {}
}
