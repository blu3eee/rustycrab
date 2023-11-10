use crate::{ utilities::app_error::AppError, BotId };
use discord::{ self, model::{ ChannelId, Message, MessageId, Event }, Discord, Connection, State };
use sea_orm::DatabaseConnection;

use super::{
    MessageContent,
    events::process_events,
    utils::messages::{ send_message, edit_message },
};

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

    pub async fn listen_to_events(&mut self) {
        loop {
            match self.connection.recv_event() {
                Ok(event) => {
                    self.state.update(&event);
                    self.process_events(&event).await;
                }
                // Handle other cases as before
                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    break;
                }
                Err(err) => {
                    println!("Receive error: {:?}", err);
                    break;
                }
            }
        }
    }
    // Add method to handle events
    pub async fn process_events(&mut self, event: &Event) {
        let _ = process_events(self, event).await;
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
}
