use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        discord_client::{ DiscordClient, MessageContent },
        commands::context::context_command::GuildConfigModel,
        messages::DiscordEmbed,
    },
    utilities::{ app_error::BoxedError, utils::process_placeholders_sync },
    queries::{
        auto_responses_queries::AutoResponsesQueries,
        message_queries::MessageQueries,
        message_embed_queries::MessageEmbedQueries,
    },
    database::messages,
    default_queries::DefaultSeaQueries,
};

pub async fn check_autores(
    client: DiscordClient,
    msg: &MessageCreate,
    config: &GuildConfigModel
) -> Result<(), BoxedError> {
    // println!("checking autores");
    let bot = client.get_bot().await?;

    let guild_id = msg.guild_id.ok_or(
        client.get_locale_string(&config.locale, "feature-guildonly", None)
    )?;

    let guild = client.get_guild(guild_id).await?;

    let bot_discord_id = bot.id.to_string();
    let guild_discord_id = guild_id.to_string();

    // println!("finding trigger");
    let autores = AutoResponsesQueries::find_by_trigger(
        &client.db,
        &bot_discord_id,
        &guild_discord_id,
        &msg.content
    ).await?;
    // println!("autores {:?}", autores);
    // println!("finding message_model");
    let message_model = MessageQueries::find_by_id(&client.db, autores.response_id).await?;
    // println!("build message_content");
    let message_content = build_response(
        &client,
        message_model,
        &Some(guild),
        &Some(msg.author.clone())
    ).await?.ok_or("can't build response from autores object")?;
    // println!("reply_message");
    let _ = client.reply_message(msg.channel_id, msg.id, message_content).await?;

    Ok(())
}

pub async fn build_response(
    client: &DiscordClient,
    message_details: messages::Model,
    guild: &Option<twilight_model::guild::Guild>,
    user: &Option<twilight_model::user::User>
) -> Result<Option<MessageContent>, BoxedError> {
    let message_type = message_details.r#type.to_lowercase();

    let text_content = message_details.content
        .filter(|content| !content.trim().is_empty())
        .map(|content| process_placeholders_sync(content, guild, user));

    let embed = (async {
        if let Some(embed_id) = message_details.embed_id {
            if let Ok(embed_model) = MessageEmbedQueries::find_by_id(&client.db, embed_id).await {
                let embed = DiscordEmbed::from(embed_model).process_placeholders(
                    &client.http,
                    guild.as_ref().map(|guild| guild.id),
                    user.as_ref().map(|user| user.id)
                ).await;
                return Some(embed);
            }
        }
        None
    }).await;

    match message_type.as_str() {
        "message" | "text" | "1" => Ok(text_content.map(MessageContent::Text)),
        "embed" | "2" => Ok(embed.map(|embed| MessageContent::DiscordEmbeds(vec![embed]))),
        "embed and text" | "3" =>
            Ok(match (text_content, embed) {
                (Some(text), Some(embed)) => {
                    if embed.is_empty() {
                        Some(MessageContent::Text(text))
                    } else {
                        Some(MessageContent::TextAndDiscordEmbeds(text, vec![embed]))
                    }
                }
                (Some(text), None) => Some(MessageContent::Text(text)),
                (None, Some(embed)) => {
                    if embed.is_empty() {
                        None
                    } else {
                        Some(MessageContent::DiscordEmbeds(vec![embed]))
                    }
                }
                (None, None) => None,
            }),
        _ => Ok(None),
    }
}
