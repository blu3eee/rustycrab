use discord::model::{ Message, Channel };
use crate::{
    utilities::app_error::AppError,
    queries::guild_config_queries,
    discordrs::{
        client::DiscordClient,
        commands::{ process_context_commands, context::dispatcher::ContextCommandDispatcher },
    },
};

pub async fn message_create(
    client: &mut DiscordClient,
    message: &Message,
    command_dispatcher: &ContextCommandDispatcher
) -> Result<(), AppError> {
    if message.author.id.to_string() == client.bot_id {
        return Ok(());
    }
    println!("{:?}", message.channel_id);

    // Asynchronously get the channel information.
    let channel: Channel = client.discord
        .get_channel(message.channel_id)
        .map_err(|e| AppError::internal_server_error(format!("Failed to get channel: {}", e)))?;

    match channel {
        Channel::Public(channel) if channel.kind == discord::model::ChannelType::Text => {
            // Handle only public text channels
            println!("Public text channel");
            handle_public_text_channel(
                client,
                message,
                channel.server_id,
                command_dispatcher
            ).await?;
        }
        Channel::Private(_) => {
            // Handle private channels, if necessary
            println!("Private channel");
        }
        _ => {
            // Ignore other channel types
        }
    }
    Ok(())
}

async fn handle_public_text_channel(
    client: &mut DiscordClient,
    message: &Message,
    server_id: discord::model::ServerId,
    command_dispatcher: &ContextCommandDispatcher
) -> Result<(), AppError> {
    let config = guild_config_queries::get_one_config(
        &client.db,
        &client.bot_id,
        &server_id.to_string()
    ).await?;

    if message.content.trim().starts_with(&config.prefix) {
        let _ = process_context_commands(client, &config, message, command_dispatcher).await;
    }
    Ok(())
}
