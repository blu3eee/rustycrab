// src/twilightrs/events/mod.rs
mod message_create;

use std::{ error::Error, sync::Arc };
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{ Event, Shard };

use self::message_create::handle_message_create;

use super::{ client::DiscordClient, dispatchers::ClientDispatchers };

pub async fn handle_bot_events(mut shard: Shard, cache: InMemoryCache, client: DiscordClient) {
    // Using Arc
    // Wrap dispatchers it in an Arc (Atomic Reference Counted). This allows multiple tasks to share ownership of dispatchers safely
    // Arc is a common pattern in Rust for sharing data across asynchronous tasks when cloning is not feasible or too expensive.
    // We don't want to create dispatcher every single time a new event is received, so this approach might be a good one
    let dispatchers: Arc<ClientDispatchers> = Arc::new(ClientDispatchers::new());

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
        // Update the cache.
        cache.update(&event);

        // Spawn a new task to handle the event
        let dispatchers_clone: Arc<ClientDispatchers> = dispatchers.clone();
        tokio::spawn(handle_event(client.clone(), dispatchers_clone, event));
    }
}

async fn handle_event(
    client: DiscordClient,
    dispatchers: Arc<ClientDispatchers>,
    event: Event
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(message_create) => {
            if let Err(e) = handle_message_create(&client, dispatchers, &message_create).await {
                eprintln!("Error handling MessageCreate event: {}", e);
            }
        }
        Event::Ready(ready) => {
            println!("[{}#{:04}] Shard is ready", ready.user.name, ready.user.discriminator);
        }
        _ => {}
    }

    Ok(())
}
