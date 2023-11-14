// src/twilightrs/events/mod.rs
mod message_create;
mod message_delete;

use std::{ error::Error, sync::Arc };
use twilight_gateway::{ Event, Shard };

use self::{ message_create::handle_message_create, message_delete::handle_message_delete };

use super::discord_client::DiscordClient;

pub async fn handle_bot_events(mut shard: Shard, client: Arc<DiscordClient>) {
    // Using Arc
    // Wrap dispatchers it in an Arc (Atomic Reference Counted). This allows multiple tasks to share ownership of dispatchers safely
    // Arc is a common pattern in Rust for sharing data across asynchronous tasks when cloning is not feasible or too expensive.
    // We don't want to create dispatcher every single time a new event is received, so this approach might be a good one

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        // Spawn a new task to handle the event

        if let Event::MessageDelete(_) = &event {
            tokio::spawn(handle_event(client.clone(), event));
        } else {
            // Update the cache.
            client.cache.update(&event);
            tokio::spawn(handle_event(client.clone(), event));
        }
    }
}

async fn handle_event(
    client: Arc<DiscordClient>,
    event: Event
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(message_create) => {
            if let Err(e) = handle_message_create(&client, &message_create).await {
                eprintln!("Error handling MessageCreate event: {}", e);
            }
        }
        Event::Ready(ready) => {
            println!("[{}#{:04}] Shard is ready", ready.user.name, ready.user.discriminator);
        }
        Event::MessageDelete(message_delete) => {
            handle_message_delete(&client, &message_delete).await?;
        }
        _ => {}
    }

    Ok(())
}
