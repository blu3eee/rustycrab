use std::error::Error;

use twilight_model::gateway::payload::incoming::MessageDelete;

use crate::twilightrs::discord_client::DiscordClient;

pub async fn handle_message_delete(
    client: DiscordClient,
    event: &MessageDelete
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut messages = client.deleted_messages.write().unwrap(); // Acquire a write lock

    // Check if the deleted message is in the cache
    if let Some(cached_message) = client.cache.message(event.id) {
        // Store the deleted message in the map
        messages
            .entry(event.channel_id)
            .or_insert_with(Vec::new)
            .push(cached_message.value().clone());

        // println!("saved deleted cached message {}", cached_message.value().content());
    } else {
        // println!("can't find cached message with id {}", event.id.to_string());
    }

    Ok(())
}
