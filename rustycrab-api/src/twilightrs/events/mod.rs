// src/twilightrs/events/mod.rs
use std::{ error::Error, sync::Arc };
use twilight_gateway::{ Event, Shard, stream::ShardEventStream };

use crate::spawn;
use futures::StreamExt;

use self::{
    message_create::handle_message_create,
    message_delete::handle_message_delete,
    interaction_handlers::handle_interaction_create,
    voice_updates::handle_voice_state_update,
};

use super::{ discord_client::DiscordClient, dispatchers::ClientDispatchers };

mod message_create;
mod message_delete;
mod interaction_handlers;

mod voice_updates;
pub async fn handle_bot_events(
    mut shards: Vec<Shard>,
    client: DiscordClient
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Using Arc
    // Wrap dispatchers it in an Arc (Atomic Reference Counted). This allows multiple tasks to share ownership of dispatchers safely
    // Arc is a common pattern in Rust for sharing data across asynchronous tasks when cloning is not feasible or too expensive.
    // We don't want to create dispatcher every single time a new event is received, so this approach might be a good one
    let dispatchers = Arc::new(ClientDispatchers::new());
    let mut stream: ShardEventStream<'_> = ShardEventStream::new(shards.iter_mut());
    loop {
        let event = match stream.next().await {
            Some((_, Ok(event))) => event,
            Some((_, Err(source))) => {
                // tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
            None => {
                break;
            }
        };

        if let Event::MessageDelete(_) = &event {
        } else {
            // Update the cache.
            client.cache.update(&event);
        }
        client.standby.process(&event);
        client.voice_music_manager.songbird.process(&event).await;

        spawn(handle_event(Arc::clone(&client), event.clone(), dispatchers.clone()));
    }

    Ok(())
}

async fn handle_event(
    client: DiscordClient,
    event: Event,
    dispatchers: Arc<ClientDispatchers>
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // event logger

    let handle_event_result = match event.clone() {
        Event::MessageCreate(message_create) => {
            handle_message_create(client, &message_create, &dispatchers).await
        }
        Event::Ready(ready) => {
            println!("[{}#{:04}] Shard is ready", ready.user.name, ready.user.discriminator);
            println!(
                "[{}#{:04}] Registering slash commands",
                ready.user.name,
                ready.user.discriminator
            );
            let _ = dispatchers.slash_commands.register_commands(Arc::clone(&client)).await;

            Ok(())
        }
        Event::MessageDelete(message_delete) => {
            handle_message_delete(Arc::clone(&client), &message_delete).await
        }
        Event::InteractionCreate(interaction) => {
            handle_interaction_create(Arc::clone(&client), &interaction, &dispatchers).await
        }
        Event::VoiceStateUpdate(update) => { handle_voice_state_update(client, &update).await }
        _ => { Ok(()) }
    };

    // handler event error
    if let Err(e) = handle_event_result {
        {
            eprintln!("Error handling event {event:?}: {e:?}");
        }
    }

    Ok(())
}
