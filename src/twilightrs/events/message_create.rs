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
    },
    unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries,
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
        let bot_id: String = client.http.current_user().await?.model().await?.id.get().to_string();
        if bot_id == msg.author.id.get().to_string() {
            return Ok(());
        }

        let guild_id_str: String = guild_id.get().to_string();

        let config = GuildConfigQueries::find_by_discord_ids(
            &client.db,
            &bot_id,
            &guild_id_str
        ).await?;
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

        let _ = check_afk(Arc::clone(&client), &config, msg, guild_id).await;
    }

    Ok(())
}
