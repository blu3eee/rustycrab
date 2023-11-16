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

#[async_trait]
pub trait BotGuildEntityQueries: DefaultSeaQueries {
    // Assumes that the implementing entity has `BotId` and `GuildId` columns.
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
