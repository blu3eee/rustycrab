// src/twilightrs/events/message_create.rs
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::error::Error;

use crate::{
    queries::guild_config_queries,
    twilightrs::{ commands, discord_client::DiscordClient },
};

pub async fn handle_message_create(
    client: &DiscordClient,
    msg: &MessageCreate
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Implement your logic for handling the message create event
    // For example, send a response message
    // check for commands

    if let Some(guild_id) = msg.guild_id {
        let bot_id: String = client.http.current_user().await?.model().await?.id.get().to_string();
        if bot_id == msg.author.id.get().to_string() {
            return Ok(());
        }

        let guild_id: String = guild_id.get().to_string();

        let config = guild_config_queries::get_one_config(&client.db, &bot_id, &guild_id).await?;
        // println!("{}", config.prefix);

        let content = msg.content.trim().to_string();

        let command_prefix = if content.starts_with(&config.prefix) {
            Some(config.prefix.clone())
        } else if content.starts_with(&format!("<@{}>", bot_id)) {
            Some(format!("<@{}>", bot_id))
        } else {
            None
        };

        if let Some(prefix) = command_prefix {
            if let Some(stripped) = content.strip_prefix(&prefix) {
                let parts: Vec<&str> = stripped
                    .trim_start_matches(' ')
                    .split_whitespace()
                    .collect();
                if let Some((&cmd_name, cmd_args)) = parts.split_first() {
                    let _ = commands::context_commands_handler(
                        client,
                        &config,
                        msg,
                        cmd_name,
                        cmd_args
                    ).await;
                }
            }
        }
    }

    Ok(())
}
