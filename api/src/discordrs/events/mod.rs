use discord::model::Event;

use super::{ client::DiscordClient, commands::context::dispatcher::ContextCommandDispatcher };

pub mod message_create;

pub async fn process_events(
    client: &mut DiscordClient,
    event: &Event,
    command_dispatcher: &ContextCommandDispatcher
) {
    match event {
        Event::MessageCreate(message) => {
            // Call the appropriate function to handle message creation
            // For example, you might dispatch to a command handler based on the message content
            // self.handle_message_create(message).await;
            let _ = message_create::message_create(client, message, command_dispatcher).await;
        }
        // Handle other events as needed
        _ => {}
    }
}
