use sea_orm::{ DatabaseConnection, Set, RelationTrait };
use async_trait::async_trait;

use crate::bot_guild_entity_queries::BotGuildEntityQueries;
use crate::default_queries::DefaultSeaQueries;
use crate::queries::bot_queries::BotQueries;
use crate::{
    database::log_settings::{
        self,
        Model as LogSettingModel,
        Entity as LogSettings,
        ActiveModel as LogSettingActiveModel,
    },
    routes::bot_logs::{ RequestCreateLogSetting, RequestUpdateLogSetting },
    utilities::app_error::AppError,
};

pub struct LogSettingQueries;

impl BotGuildEntityQueries for LogSettingQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        log_settings::Relation::Bots.def()
    }
    fn guild_relation() -> sea_orm::entity::RelationDef {
        log_settings::Relation::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for LogSettingQueries {
    type Entity = LogSettings;
    type ActiveModel = LogSettingActiveModel;

    type CreateDto = RequestCreateLogSetting;
    type UpdateDto = RequestUpdateLogSetting;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateDto
    ) -> Result<LogSettingModel, AppError> {
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

        let active_model = LogSettingActiveModel {
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
        if let Some(value) = update_data.specify_channels {
            active_model.specify_channels = Set(value);
        }

        if let Some(value) = update_data.new_account_age {
            active_model.new_account_age = Set(value);
        }

        Ok(())
    }
}
