// queries/bot_queries.rs
use sea_orm::{ ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set };

use crate::{
    database::bots::{ self, Entity as Bots, Model as BotModel },
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
    routes::bots::{ RequestUpdateBot, RequestCreateBot },
};

pub async fn create_bot(
    db: &DatabaseConnection,
    dto: RequestCreateBot
) -> Result<BotModel, AppError> {
    // Check if a bot exists with the given bot_id
    match Bots::find().filter(crate::database::bots::Column::BotId.eq(&dto.bot_id)).one(db).await {
        Ok(Some(bot)) => {
            // Bot already exists, return it with a Found status
            Ok(bot)
        }
        Ok(None) => {
            // No bot exists, create a new bot
            let new_bot = bots::ActiveModel {
                bot_id: Set(dto.bot_id),
                token: Set(dto.token),
                theme_hex_color: Set(dto.theme_hex_color.unwrap_or_default()),
                discord_secret: Set(dto.discord_secret.unwrap_or_default()),
                discord_callback_url: Set(dto.discord_callback_url.unwrap_or_default()),
                ..Default::default() // Set other fields to default if necessary
            };

            let bot = new_bot.insert(db).await.map_err(convert_seaorm_error)?;

            // Bot created, return it with a Created status
            Ok(bot)
        }
        Err(err) => {
            // Error querying the database
            Err(AppError::internal_server_error(format!("Database error: {}", err)))
        }
    }
}

pub async fn update_bot(
    db: &DatabaseConnection,
    id: &i32, // Assuming bot ID is passed as an i32 directly.
    dto: RequestUpdateBot
) -> Result<BotModel, AppError> {
    // Fetch the bot by bot_id to update
    let mut bot: bots::ActiveModel = get_bot(db, id).await?.into();

    // Apply updates from the DTO
    if let Some(token) = dto.token {
        bot.token = Set(token);
    }
    if let Some(theme_hex_color) = dto.theme_hex_color {
        bot.theme_hex_color = Set(theme_hex_color);
    }
    if let Some(discord_secret) = dto.discord_secret {
        bot.discord_secret = Set(discord_secret);
    }
    if let Some(discord_callback_url) = dto.discord_callback_url {
        bot.discord_callback_url = Set(discord_callback_url);
    }
    if let Some(premium_flags) = dto.premium_flags {
        bot.premium_flags = Set(premium_flags);
    }

    // Update the bot in the database
    // Save the updated bot back to the database
    bot.update(db).await.map_err(|err| {
        eprintln!("Error updating bot: {:?}", err);
        AppError::internal_server_error("There was an error updating the bot")
    })
}

pub async fn update_bot_from_discord_id(
    db: &DatabaseConnection,
    bot_id: &str,
    dto: RequestUpdateBot
) -> Result<BotModel, AppError> {
    // Fetch the bot by bot_id to update
    let mut bot: bots::ActiveModel = get_bot_from_discord_id(db, bot_id).await?.into();

    // Apply updates from the DTO
    if let Some(token) = dto.token {
        bot.token = Set(token);
    }
    if let Some(theme_hex_color) = dto.theme_hex_color {
        bot.theme_hex_color = Set(theme_hex_color);
    }
    if let Some(discord_secret) = dto.discord_secret {
        bot.discord_secret = Set(discord_secret);
    }
    if let Some(discord_callback_url) = dto.discord_callback_url {
        bot.discord_callback_url = Set(discord_callback_url);
    }
    if let Some(premium_flags) = dto.premium_flags {
        bot.premium_flags = Set(premium_flags);
    }

    // Update the bot in the database
    // Save the updated bot back to the database
    bot.update(db).await.map_err(|err| {
        eprintln!("Error updating bot: {:?}", err);
        AppError::internal_server_error("There was an error updating the bot")
    })
}

pub async fn get_all_bots(db: &DatabaseConnection) -> Result<Vec<BotModel>, AppError> {
    // -> Result<Vec<Bots>, AppError>
    Bots::find()
        .all(db).await
        .map_err(|err| {
            eprintln!("Error getting all bots: {:?}", err);
            AppError::internal_server_error("There was an error getting all bots")
        })
}

pub async fn get_bot(db: &DatabaseConnection, id: &i32) -> Result<BotModel, AppError> {
    Bots::find()
        .filter(crate::database::bots::Column::Id.eq(*id))
        .one(db).await
        .map_err(convert_seaorm_error)
        .and_then(|bot| bot.ok_or_else(|| AppError::not_found("Bot not found")))
}

pub async fn get_bot_from_discord_id(
    db: &DatabaseConnection,
    bot_id: &str
) -> Result<BotModel, AppError> {
    Bots::find()
        .filter(crate::database::bots::Column::BotId.eq(bot_id))
        .one(db).await
        .map_err(convert_seaorm_error)
        .and_then(|bot| bot.ok_or_else(|| AppError::not_found("Bot not found")))
}
