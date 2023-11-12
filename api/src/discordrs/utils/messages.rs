use discord::model::{ ChannelId, Message, MessageId };

use crate::{ discordrs::{ client::DiscordClient, MessageContent }, utilities::app_error::AppError };

use super::build_embed::build_embed;

pub fn send_message(
    client: &DiscordClient,
    channel_id: ChannelId,
    message_content: MessageContent
) -> Result<Message, AppError> {
    // println!("sending message");
    match message_content {
        MessageContent::Text(text) => {
            // Send a text message
            return client.discord
                .send_message(channel_id, &text, "", false)
                .map_err(AppError::from);
        }
        MessageContent::Embed(embed) => {
            // Send an embed message
            println!("sending embed");
            return client.discord
                .send_embed(channel_id, "", |cur| build_embed(cur, embed))
                .map_err(AppError::from);
        }
        MessageContent::TextAndEmbed(text, embed) => {
            // Send a text message with an embed
            return client.discord
                .send_embed(channel_id, &text, |cur| build_embed(cur, embed))
                .map_err(AppError::from);
        }
        MessageContent::None => {
            // Handle the case where there is no message content
            Err(AppError::bad_request("Invalid message content"))
        }
    }

    // Err(AppError::bad_request("Invalid message content"))
}

pub fn edit_message(
    client: &mut DiscordClient,
    channel_id: ChannelId,
    message_id: MessageId,
    message_content: MessageContent
) -> Result<Message, AppError> {
    match message_content {
        MessageContent::Text(text) => {
            // Edit a text message
            client.discord.edit_message(channel_id, message_id, &text).map_err(AppError::from)
        }
        MessageContent::Embed(embed) => {
            // Edit a message to only contain an embed
            client.discord
                .edit_embed(channel_id, message_id, |cur| build_embed(cur, embed))
                .map_err(AppError::from)
        }
        MessageContent::TextAndEmbed(text, embed) => {
            // Edit a message to contain both text and an embed
            client.discord.edit_message(channel_id, message_id, &text).map_err(AppError::from)?;
            client.discord
                .edit_embed(channel_id, message_id, |cur| build_embed(cur, embed))
                .map_err(AppError::from)
        }
        MessageContent::None => {
            // Handle the case where there is no content to edit
            Err(AppError::bad_request("Invalid message content"))
        }
    }
}
