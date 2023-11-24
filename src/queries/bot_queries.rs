// queries/bot_queries.rs
use async_trait::async_trait;
use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set };

use crate::{
    database::bots::{ self, Model as BotModel },
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
    router::routes::bots::{ RequestCreateBot, RequestUpdateBot },
};

pub struct BotQueries {}

impl BotQueries {
    pub async fn find_by_discord_id(
        db: &DatabaseConnection,
        bot_discord_id: &str
    ) -> Result<BotModel, AppError> {
        bots::Entity
            ::find()
            .filter(bots::Column::BotId.eq(bot_discord_id))
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Bot not found"))
    }

    pub async fn update_by_discord_id(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        update_data: RequestUpdateBot
    ) -> Result<BotModel, AppError> {
        // Fetch the bot by bot_id to update
        let model = Self::find_by_discord_id(db, bot_discord_id).await?;
        let mut active_model: bots::ActiveModel = model.into();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }
}

#[async_trait]
impl DefaultSeaQueries for BotQueries {
    type Entity = bots::Entity;
    type ActiveModel = bots::ActiveModel;

    type CreateData = RequestCreateBot;
    type UpdateData = RequestUpdateBot;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if let Ok(model) = Self::find_by_discord_id(db, &create_data.bot_id).await {
            return Ok(model);
        }

        let active_model = bots::ActiveModel {
            bot_id: Set(create_data.bot_id),
            token: Set(create_data.token),
            theme_hex_color: Set(create_data.theme_hex_color.unwrap_or_default()),
            discord_secret: Set(create_data.discord_secret.unwrap_or_default()),
            discord_callback_url: Set(create_data.discord_callback_url.unwrap_or_default()),
            ..Default::default() // Set other fields to default if necessary
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        // Apply updates from the DTO
        if let Some(token) = update_data.token {
            active_model.token = Set(token);
        }
        if let Some(theme_hex_color) = update_data.theme_hex_color {
            active_model.theme_hex_color = Set(theme_hex_color);
        }
        if let Some(discord_secret) = update_data.discord_secret {
            active_model.discord_secret = Set(discord_secret);
        }
        if let Some(discord_callback_url) = update_data.discord_callback_url {
            active_model.discord_callback_url = Set(discord_callback_url);
        }
        if let Some(premium_flags) = update_data.premium_flags {
            active_model.premium_flags = Set(premium_flags);
        }

        Ok(())
    }
}
