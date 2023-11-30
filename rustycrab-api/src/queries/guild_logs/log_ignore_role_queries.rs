use rustycrab_model::response::logs::ignore::{
    RequestCreateLogIgnoreRole,
    RequestUpdateLogIgnoreRole,
};
use sea_orm::{
    DatabaseConnection,
    Set,
    EntityTrait,
    QueryFilter,
    ColumnTrait,
    Condition,
    QuerySelect,
    RelationTrait,
    JoinType::LeftJoin,
};
use async_trait::async_trait;

use crate::{
    database::{
        log_ignore_roles::{
            self,
            Entity as LogIgnoreRoles,
            ActiveModel as LogIgnoreRoleActiveModel,
        },
        log_settings,
    },
    default_queries::DefaultSeaQueries,
    utilities::app_error::AppError,
};
use super::log_setting_queries::LogSettingQueries;

pub struct LogIgnoreRoleQueries {}

impl LogIgnoreRoleQueries {
    pub async fn check_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        role_discord_id: &str
    ) -> Result<bool, AppError> {
        if
            let Ok(Some(_)) = <<Self as DefaultSeaQueries>::Entity as EntityTrait>
                ::find()
                .join(LeftJoin, log_ignore_roles::Relation::LogSettings.def())
                .join(LeftJoin, log_settings::Relation::Bots.def())
                .join(LeftJoin, log_settings::Relation::GuildInfo.def())
                .filter(
                    Condition::all()
                        .add(crate::database::bots::Column::BotId.eq(bot_discord_id))
                        .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id))
                        .add(log_ignore_roles::Column::RoleId.eq(role_discord_id))
                )
                .one(db).await
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub async fn get_guild_ignores_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(LeftJoin, log_ignore_roles::Relation::LogSettings.def())
            .join(LeftJoin, log_settings::Relation::Bots.def())
            .join(LeftJoin, log_settings::Relation::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(crate::database::bots::Column::BotId.eq(bot_discord_id))
                    .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .all(db).await
            .map_err(AppError::from)
    }

    pub async fn get_guild_ignores(
        db: &DatabaseConnection,
        log_setting_id: i32
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .filter(
                <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Column::LogSettingId.eq(
                    log_setting_id
                )
            )
            .all(db).await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl DefaultSeaQueries for LogIgnoreRoleQueries {
    type Entity = LogIgnoreRoles;

    type ActiveModel = LogIgnoreRoleActiveModel;

    type CreateData = RequestCreateLogIgnoreRole;
    type UpdateData = RequestUpdateLogIgnoreRole;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if
            let Ok(Some(model)) = LogIgnoreRoles::find()
                .filter(
                    Condition::all()
                        .add(log_ignore_roles::Column::LogSettingId.eq(create_data.log_setting_id))
                        .add(log_ignore_roles::Column::RoleId.eq(&create_data.role_id))
                )
                .one(db).await
        {
            return Ok(model);
        }
        let log_setting = LogSettingQueries::find_by_id(db, create_data.log_setting_id).await?;
        Self::save_active_model(db, Self::ActiveModel {
            log_setting_id: Set(log_setting.id),
            role_id: Set(create_data.role_id),
            ..Default::default()
        }).await
    }

    #[allow(unused_variables)]
    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        Ok(())
    }
}
