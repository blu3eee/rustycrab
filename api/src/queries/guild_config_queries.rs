use async_trait::async_trait;
// queries/guild_config_queries.rs
use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, RelationTrait };

use crate::{
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
    database::{
        bot_guild_configurations::{
            self,
            Entity as GuildConfigs,
            Model as GuildConfigModel,
            ActiveModel as GuildConfigActiveModel,
        },
        bots,
    },
    bot_guild_entity_queries::BotGuildEntityQueries,
    default_queries::DefaultSeaQueries,
    routes::bot_guild_configs::{ RequestCreateConfig, RequestUpdateConfig },
};

use super::bot_queries::BotQueries;

pub struct GuildConfigQueries {}

impl GuildConfigQueries {
    pub async fn get_all_bot_configs(
        db: &DatabaseConnection,
        bot_id: &str
    ) -> Result<Vec<GuildConfigModel>, AppError> {
        // Find the configuration for the given bot_id and guild_id.
        GuildConfigs::find()
            .filter(bots::Column::BotId.eq(bot_id))
            .all(db).await
            .map_err(convert_seaorm_error)
    }

    // pub async fn test_find_by_discord_ids(
    //     db: &DatabaseConnection,
    //     bot_discord_id: &str,
    //     guild_discord_id: &str
    // ) -> Result<GuildConfigModel, AppError> {
    //     println!(
    //         "BotGuildEntityQueries find_by_discord_ids {} {}",
    //         bot_discord_id,
    //         guild_discord_id
    //     );

    //     GuildConfigs::find()
    //         .join(sea_orm::JoinType::LeftJoin, bot_guild_configurations::Relation::Bots.def())
    //         .join(sea_orm::JoinType::LeftJoin, bot_guild_configurations::Relation::GuildInfo.def())
    //         .filter(
    //             Condition::all()
    //                 .add(crate::database::bots::Column::BotId.eq(bot_discord_id)) // Adjust 'BotId' if necessary
    //                 .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id)) // Adjust 'GuildId' if necessary
    //         )
    //         .one(db).await
    //         .map_err(convert_seaorm_error)?
    //         .ok_or_else(|| AppError::not_found("Record not found"))
    // }
}

impl BotGuildEntityQueries for GuildConfigQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        bot_guild_configurations::Relation::Bots.def()
    }
    fn guild_relation() -> sea_orm::entity::RelationDef {
        bot_guild_configurations::Relation::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for GuildConfigQueries {
    type Entity = GuildConfigs;
    type ActiveModel = GuildConfigActiveModel;

    type CreateDto = RequestCreateConfig;
    type UpdateDto = RequestUpdateConfig;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateDto
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if
            let Ok(model) = Self::find_by_discord_ids(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id
            ).await
        {
            return Ok(model);
        }
        let guild = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;

        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;

        let active_model = Self::ActiveModel {
            bot_id: Set(Some(bot.id)),
            guild_id: Set(Some(guild.id)),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateDto
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.locale {
            active_model.locale = Set(value);
        }

        if let Some(value) = update_data.prefix {
            active_model.prefix = Set(value);
        }

        if let Some(value) = update_data.module_flags {
            active_model.module_flags = Set(value);
        }

        if let Some(value) = update_data.premium_flags {
            active_model.premium_flags = Set(value);
        }

        Ok(())
    }
}
