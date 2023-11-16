use crate::utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error };
use async_trait::async_trait;

use sea_orm::{
    IntoActiveModel,
    DatabaseConnection,
    EntityTrait,
    Condition,
    QueryFilter,
    ColumnTrait,
};

use super::default_queries::DefaultSeaQueries;

/// `BotGuildEntityQueries` is an extension of `DefaultSeaQueries` tailored for entities
/// that are associated with both a bot and a guild in Discord. It provides methods
/// for finding and updating these entities based on bot and guild Discord IDs.
#[async_trait]
#[async_trait]
pub trait BotGuildEntityQueries: DefaultSeaQueries {
    /// Finds an entity based on both the bot's and guild's Discord IDs.
    ///
    /// This method assumes that the implementing entity has columns `BotId` and `GuildId`,
    /// and it performs a query to retrieve the entity that matches both IDs.
    ///
    /// # Parameters
    /// - `db`: The database connection.
    /// - `bot_discord_id`: The Discord ID of the bot.
    /// - `guild_discord_id`: The Discord ID of the guild.
    ///
    /// # Returns
    /// A result containing either the entity model if found, or an `AppError` if not found
    /// or if an error occurs during the query.
    async fn find_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        // This is a generic implementation. Specific column names should be defined in the actual entity.
        Self::Entity::find()
            .filter(
                Condition::all()
                    .add(crate::database::bots::Column::BotId.eq(bot_discord_id))
                    .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .one(db).await
            .map_err(convert_seaorm_error)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }

    /// Updates an entity based on both the bot's and guild's Discord IDs with the provided data.
    ///
    /// This method first finds the entity matching the provided Discord IDs and then
    /// applies updates based on the given DTO. It's a convenient way to update entities
    /// that are identified by a combination of bot and guild IDs.
    ///
    /// # Parameters
    /// - `db`: The database connection.
    /// - `bot_discord_id`: The Discord ID of the bot.
    /// - `guild_discord_id`: The Discord ID of the guild.
    /// - `update_data`: Data transfer object containing the update information.
    ///
    /// # Returns
    /// A result containing either the updated entity model or an `AppError` if an error occurs.
    async fn update_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        update_data: Self::UpdateDto
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>
    {
        let model: <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::find_by_discord_ids(
            db,
            bot_discord_id,
            guild_discord_id
        ).await?;

        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into_active_model();

        Self::apply_updates(&mut active_model, update_data)?;

        Self::save_active_model(db, active_model).await
    }
}
