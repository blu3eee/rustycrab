// src/twilightrs/events/mod.rs
mod message_create;

use std::error::Error;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{ Event, Shard };

use self::message_create::handle_message_create;

use super::client::DiscordClient;

pub async fn handle_bot_events(mut shard: Shard, cache: InMemoryCache, client: DiscordClient) {
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
        tokio::spawn(handle_event(client.clone(), event));
    }
}

async fn handle_event(
    client: DiscordClient,
    event: Event
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            if let Err(e) = handle_message_create(&client, &msg).await {
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
