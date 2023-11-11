use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    Condition,
    Set,
    ActiveValue,
};
use crate::{
    database::{
        bot_guild_welcomes::{
            Entity as GuildWelcome,
            Model as GuildWelcomeModel,
            ActiveModel as GuildWelcomeActiveModel,
        },
        bots,
        guild_info,
    },
    routes::guild_welcomes::{
        RequestCreateWelcome,
        ResponseGuildWelcomeDetails,
        RequestUpdateWelcome,
    },
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
};

use super::{
    guild_queries::{ get_one_guild_or_create, get_one_guild },
    bot_queries::{ get_bot_from_discord_id, get_bot },
    save_active_model,
    message_queries::{ create_message, fetch_message_response, update_message },
};

pub async fn create_welcome(
    db: &DatabaseConnection,
    create_dto: RequestCreateWelcome
) -> Result<GuildWelcomeModel, AppError> {
    if
        let Ok(Some(welcome)) = GuildWelcome::find()
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(&create_dto.guild_discord_id))
                    .add(guild_info::Column::GuildId.eq(&create_dto.bot_discord_id))
            )

            .one(db).await
    {
        return Ok(welcome);
    }
    let guild: guild_info::Model = get_one_guild_or_create(db, &create_dto.guild_discord_id).await?;
    let bot: bots::Model = get_bot_from_discord_id(db, &create_dto.bot_discord_id).await?;
    let message: Option<crate::routes::ResponseMessage> = if
        let Some(message_data) = create_dto.message_data
    {
        Some(create_message(db, message_data).await?)
    } else {
        None
    };

    let active_model: GuildWelcomeActiveModel = GuildWelcomeActiveModel {
        bot_id: Set(Some(bot.id)),
        guild_id: Set(Some(guild.id)),
        message_id: Set(message.map(|e| e.id)),
        channel_id: Set(create_dto.channel_id),
        ..Default::default()
    };

    save_active_model(db, active_model).await
}

pub async fn get_welcome(db: &DatabaseConnection, id: &i32) -> Result<GuildWelcomeModel, AppError> {
    GuildWelcome::find_by_id(*id)
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Welcome message not found"))
}

pub async fn get_welcome_from_discord_id(
    db: &DatabaseConnection,
    bot_id: &str,
    guild_id: &str
) -> Result<GuildWelcomeModel, AppError> {
    GuildWelcome::find()
        .filter(
            Condition::all()
                .add(bots::Column::BotId.eq(bot_id))
                .add(guild_info::Column::GuildId.eq(guild_id))
        )
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Welcome message not found"))
}

pub async fn get_welcome_response(
    db: &DatabaseConnection,
    id: &i32
) -> Result<ResponseGuildWelcomeDetails, AppError> {
    let welcome: GuildWelcomeModel = get_welcome(db, id).await?;

    let guild: crate::routes::guilds::ResponseGuild = get_one_guild(
        db,
        &welcome.guild_id.unwrap()
    ).await?.into();

    let bot: crate::routes::bots::ResponseBot = get_bot(db, &welcome.bot_id.unwrap()).await?.into();

    let message = fetch_message_response(db, &welcome.message_id).await?;

    Ok(ResponseGuildWelcomeDetails {
        id: welcome.id,
        channel_id: welcome.channel_id,
        enabled: welcome.enabled,
        bot: Some(bot),
        guild: Some(guild),
        message,
    })
}

pub async fn update_welcome(
    db: &DatabaseConnection,
    id: i32,
    update_dto: RequestUpdateWelcome
) -> Result<GuildWelcomeModel, AppError> {
    let mut welcome: GuildWelcomeActiveModel = GuildWelcome::find_by_id(id)
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Welcome message not found"))?
        .into(); // Convert to ActiveModel for update

    // Update channel_id if provided
    if let Some(channel_id) = update_dto.channel_id {
        welcome.channel_id = Set(Some(channel_id));
    }

    // Handle message_data update
    if let Some(message_data) = update_dto.message_data {
        if let ActiveValue::Set(Some(message_id)) = welcome.message_id {
            let _ = update_message(db, &message_id, message_data).await?;
        } else {
            let message = create_message(db, message_data).await?;
            welcome.message_id = Set(Some(message.id));
        }
    }

    // Save the updated configuration back to the database
    save_active_model(db, welcome).await
}
