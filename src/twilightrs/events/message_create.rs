// src/twilightrs/events/message_create.rs
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, sync::Arc };

use crate::{
    queries::guild_config_queries::GuildConfigQueries,
    twilightrs::{
        commands,
        discord_client::DiscordClient,
        dispatchers::ClientDispatchers,
        utils::afk::check_afk,
        bot::auto_response::check_autores,
    },
};

pub async fn handle_message_create(
    client: DiscordClient,
    msg: &MessageCreate,
    dispatchers: &Arc<ClientDispatchers>
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Implement your logic for handling the message create event
    // For example, send a response message
    // check for commands
    if let Some(guild_id) = msg.guild_id {
        let bot = client.get_bot().await?;
        if bot.id == msg.author.id {
            return Ok(());
        }

        let config = GuildConfigQueries::get_or_create_config(
            &client.db,
            &bot.id.to_string(),
            &guild_id.get().to_string()
        ).await?;

        let content = msg.content.trim().to_string();

        let command_prefix = if content.starts_with(&config.prefix) {
            Some(config.prefix.clone())
        } else if content.starts_with(&format!("<@{}>", bot.id)) {
            Some(format!("<@{}>", bot.id))
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
                        Arc::clone(&client),
                        &config,
                        &dispatchers,
                        msg,
                        cmd_name,
                        cmd_args
                    ).await;
                }
            }
        }

        // check afk
        let _ = check_afk(Arc::clone(&client), &config, msg, guild_id).await;

        // check for auto-responses
        let _ = check_autores(Arc::clone(&client), msg, &config).await;
    }

    Ok(())
}
