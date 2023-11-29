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
    println!("checking autores");
    let bot = client.get_bot().await?;

    let guild_id = msg.guild_id.ok_or(
        client.get_locale_string(&config.locale, "feature-guildonly", None)
    )?;

    let guild = client.get_guild(guild_id).await?;

    let bot_discord_id = bot.id.to_string();
    let guild_discord_id = guild_id.to_string();

    println!("finding trigger");
    let autores = AutoResponsesQueries::find_by_trigger(
        &client.db,
        &bot_discord_id,
        &guild_discord_id,
        &msg.content
    ).await?;
    println!("autores {:?}", autores);
    if let Some(message_id) = autores.response_id {
        println!("finding message_model");
        let message_model = MessageQueries::find_by_id(&client.db, message_id).await?;
        println!("build message_content");
        let message_content = build_response(
            &client,
            message_model,
            &Some(guild),
            &Some(msg.author.clone())
        ).await?.ok_or("can't build response from autores object")?;
        println!("reply_message");
        let _ = client.reply_message(msg.channel_id, msg.id, message_content).await?;
    }

    Ok(())
}

pub async fn build_response(
    client: &DiscordClient,
    message_details: messages::Model,
    guild: &Option<twilight_model::guild::Guild>,
    user: &Option<twilight_model::user::User>
) -> Result<Option<MessageContent>, BoxedError> {
    let message: Option<MessageContent> = match message_details.r#type.to_lowercase().as_str() {
        "message" | "text" | "1" => {
            if let Some(content) = message_details.content {
                Some(MessageContent::Text(process_placeholders_sync(content, guild, user)))
            } else {
                None
            }
        }
        "embed" | "2" => {
            if let Some(embed_id) = message_details.embed_id {
                if
                    let Ok(embed_model) = MessageEmbedQueries::find_by_id(
                        &client.db,
                        embed_id
                    ).await
                {
                    Some(
                        MessageContent::DiscordEmbeds(
                            vec![
                                DiscordEmbed::from(embed_model).process_placeholders(
                                    &client.http,
                                    guild.as_ref().map(|guild| guild.id),
                                    user.as_ref().map(|user| user.id)
                                ).await
                            ]
                        )
                    )
                } else {
                    None
                }
            } else {
                None
            }
        }
        "embed and text" | "3" => {
            let text = if let Some(content) = message_details.content {
                Some(process_placeholders_sync(content, guild, user))
            } else {
                None
            };
            if let Some(embed_id) = message_details.embed_id {
                if
                    let Ok(embed_model) = MessageEmbedQueries::find_by_id(
                        &client.db,
                        embed_id
                    ).await
                {
                    let embed = DiscordEmbed::from(embed_model).process_placeholders(
                        &client.http,
                        guild.as_ref().map(|guild| guild.id),
                        user.as_ref().map(|user| user.id)
                    ).await;
                    Some(
                        text.map_or(MessageContent::DiscordEmbeds(vec![embed.clone()]), |text|
                            MessageContent::TextAndDiscordEmbeds(text, vec![embed])
                        )
                    )
                } else {
                    text.map(|text| MessageContent::Text(text))
                }
            } else {
                text.map(|text| MessageContent::Text(text))
            }
        }

        _ => { None }
    };

    return Ok(message);
}
