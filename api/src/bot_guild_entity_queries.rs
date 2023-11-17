use crate::utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error };
use async_trait::async_trait;

use sea_orm::{
    IntoActiveModel,
    DatabaseConnection,
    EntityTrait,
    Condition,
    QueryFilter,
    ColumnTrait,
    QuerySelect,
};

use super::default_queries::DefaultSeaQueries;

/// `BotGuildEntityQueries` is an extension of `DefaultSeaQueries` tailored for entities
/// that are associated with both a bot and a guild in Discord. It provides methods
/// for finding and updating these entities based on bot and guild Discord IDs.
#[async_trait]
pub trait BotGuildEntityQueries: DefaultSeaQueries {
    /// Defines the relation to the `bots` entity for entities that are associated with Discord bots.
    ///
    /// ### Returns
    /// - `sea_orm::entity::RelationDef`: The definition of the relationship between the current entity and the `bots` entity.
    ///
    /// ### Description
    /// This function should return the relation definition that links the current entity with the `bots` entity.
    /// It is used in join operations to query data based on the relationship between these entities.
    ///
    /// ### Example
    /// ```rust,ignore
    /// impl BotGuildEntityQueries for MyEntityQueries {
    ///     fn bot_relation() -> sea_orm::entity::RelationDef {
    ///         MyEntity::Relation::Bots.def()
    ///     }
    ///     // other trait methods...
    /// }
    /// ```
    /// In the above example, `MyEntity::Relation::Bots.def()` refers to the defined relation in the `MyEntity`'s model.
    fn bot_relation() -> sea_orm::entity::RelationDef;

    /// Defines the relation to the `guild_info` entity for entities that are associated with Discord guilds.
    ///
    /// ### Returns
    /// - `sea_orm::entity::RelationDef`: The definition of the relationship between the current entity and the `guild_info` entity.
    ///
    /// ### Description
    /// This function should return the relation definition that links the current entity with the `guild_info` entity.
    /// It is used in join operations to query data based on the relationship between these entities.
    ///
    /// ### Example
    /// ```rust,ignore
    /// impl BotGuildEntityQueries for MyEntityQueries {
    ///     fn guild_relation() -> sea_orm::entity::RelationDef {
    ///         MyEntity::Relation::GuildInfo.def()
    ///     }
    ///     // other trait methods...
    /// }
    /// ```
    /// Here, `MyEntity::Relation::GuildInfo.def()` refers to the defined relation in the `MyEntity`'s model.
    fn guild_relation() -> sea_orm::entity::RelationDef;

    /// Finds an entity based on both the bot's and guild's Discord IDs.
    ///
    /// This method uses the provided relations (`bot_relation` and `guild_relation`) to perform a query
    /// that retrieves the entity matching both a specific bot's and a guild's Discord IDs.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `bot_discord_id`: The Discord ID of the bot.
    /// - `guild_discord_id`: The Discord ID of the guild.
    ///
    /// ### Returns
    /// - `Result<<Self::Entity as EntityTrait>::Model, AppError>`: A result containing either the found entity model, or an `AppError` if not found or in case of a query error.
    ///
    /// ### Description
    /// The method performs a database query using left joins on the `bot_relation` and `guild_relation` to find an entity.
    /// The query filters entities based on the matching `BotId` and `GuildId`, which correspond to the provided Discord IDs.
    /// It is a versatile method for querying entities related to specific bots and guilds in a Discord context.
    ///
    /// ### Implementation Notes
    /// Implementing entities should have defined relations with `bots` and `guild_info` entities to use this function properly and effectively.
    async fn find_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        Self::Entity::find()
            .join(sea_orm::JoinType::LeftJoin, Self::bot_relation())
            .join(sea_orm::JoinType::LeftJoin, Self::guild_relation())
            .filter(
                Condition::all()
                    .add(crate::database::bots::Column::BotId.eq(bot_discord_id)) // Adjust 'BotId' if necessary
                    .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id)) // Adjust 'GuildId' if necessary
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
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `bot_discord_id`: The Discord ID of the bot.
    /// - `guild_discord_id`: The Discord ID of the guild.
    /// - `update_data`: Data transfer object containing the update information.
    ///
    /// ### Returns
    /// A result containing either the updated entity model or an `AppError` if an error occurs.
    ///
    /// ### Implementation Notes
    /// Implementing entities should have defined relations with `bots` and `guild_info` entities to use this function properly and effectively.
    async fn update_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        update_data: Self::UpdateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>
    {
        let model: <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::find_by_discord_ids(
            db,
            bot_discord_id,
            guild_discord_id
        ).await?;

        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into_active_model();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }
}
