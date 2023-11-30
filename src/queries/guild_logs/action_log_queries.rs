use async_trait::async_trait;
use rustycrab_model::response::logs::action_log::{ RequestCreateLogAction, RequestUpdateActionLog };
use sea_orm::{
    EntityTrait,
    DatabaseConnection,
    RelationTrait,
    QueryFilter,
    Condition,
    QuerySelect,
    ColumnTrait,
    Set,
};

use crate::{
    default_queries::DefaultSeaQueries,
    database::guild_action_logs::{
        self,
        Entity as ActionLogs,
        ActiveModel as ActionLogActiveModel,
        Relation as ActionLogsRelations,
    },
    utilities::app_error::AppError,
    queries::{ bot_queries::BotQueries, guild_queries::GuildQueries },
};

pub struct ActionLogsQueries;

impl ActionLogsQueries {
    pub async fn find_unique(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        channel_discord_id: &str
    ) -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(sea_orm::JoinType::LeftJoin, ActionLogsRelations::Bots.def())
            .join(sea_orm::JoinType::LeftJoin, ActionLogsRelations::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(crate::database::bots::Column::BotId.eq(bot_discord_id))
                    .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id))
                    .add(guild_action_logs::Column::ChannelId.eq(channel_discord_id))
            )
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }

    pub async fn find_guild_action_logs(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(sea_orm::JoinType::LeftJoin, ActionLogsRelations::Bots.def())
            .join(sea_orm::JoinType::LeftJoin, ActionLogsRelations::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(crate::database::bots::Column::BotId.eq(bot_discord_id))
                    .add(crate::database::guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .all(db).await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl DefaultSeaQueries for ActionLogsQueries {
    type Entity = ActionLogs;
    type ActiveModel = ActionLogActiveModel;

    type CreateData = RequestCreateLogAction;
    type UpdateData = RequestUpdateActionLog;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        // Self::save_active_model(db, active_model).await
        if
            let Ok(model) = Self::find_unique(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id,
                &create_data.channel_id
            ).await
        {
            let mut active_model: ActionLogActiveModel = model.into();
            active_model.events = Set(create_data.events);

            return Self::save_active_model(db, active_model).await;
        }

        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.bot_discord_id).await?;

        let active_model = Self::ActiveModel {
            bot_id: Set(Some(bot.id)),
            guild_id: Set(Some(guild.id)),
            channel_id: Set(create_data.channel_id),
            events: Set(create_data.events),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.channel_id {
            active_model.channel_id = Set(value);
        }

        if let Some(value) = update_data.events {
            active_model.events = Set(value);
        }

        Ok(())
    }
}
