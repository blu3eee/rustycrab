use async_trait::async_trait;
use rustycrab_model::response::auto_response::{
    RequestCreateAutoResponse,
    RequestUpdateAutoResponse,
};
use sea_orm::{
    DatabaseConnection,
    Set,
    EntityTrait,
    QueryFilter,
    Condition,
    QuerySelect,
    ColumnTrait,
    RelationTrait,
    ActiveValue,
    IntoActiveModel,
    PrimaryKeyTrait,
    DeleteResult,
};

use crate::{
    default_queries::DefaultSeaQueries,
    database::{
        auto_responses::{ Entity as AutoResponses, ActiveModel as AutoResActiveModel, self },
        bots,
        guild_info,
    },
    utilities::app_error::AppError,
    multi_bot_guild_entities_queries::MultipleBotGuildEntityQueries,
};

use super::{
    bot_queries::BotQueries,
    guild_queries::GuildQueries,
    message_queries::MessageQueries,
};

pub struct AutoResponsesQueries {}

impl AutoResponsesQueries {
    pub async fn find_by_trigger(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        trigger: &str
    ) -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(
                sea_orm::JoinType::LeftJoin,
                <Self as MultipleBotGuildEntityQueries>::bot_relation()
            )
            .join(
                sea_orm::JoinType::LeftJoin,
                <Self as MultipleBotGuildEntityQueries>::guild_relation()
            )
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(guild_info::Column::GuildId.eq(guild_discord_id))
                    .add(auto_responses::Column::Trigger.eq(trigger.trim()))
            )
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Auto-response not found"))
    }

    pub async fn update_by_trigger(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        trigger: &str,
        update_data: <Self as DefaultSeaQueries>::UpdateData
    )
        -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError>
        where
            <<<Self as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<Self as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::find_by_trigger(
            db,
            bot_discord_id,
            guild_discord_id,
            trigger
        ).await?;

        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into_active_model();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }
}

impl MultipleBotGuildEntityQueries for AutoResponsesQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        auto_responses::Relation::Bots.def()
    }

    fn guild_relation() -> sea_orm::entity::RelationDef {
        auto_responses::Relation::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for AutoResponsesQueries {
    type Entity = AutoResponses;
    type ActiveModel = AutoResActiveModel;

    type CreateData = RequestCreateAutoResponse;
    type UpdateData = RequestUpdateAutoResponse;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if
            let Ok(entity) = Self::find_by_trigger(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id,
                &create_data.trigger
            ).await
        {
            return Ok(entity);
        }

        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.guild_discord_id).await?;

        let message = MessageQueries::create_entity(db, create_data.response_data).await?;

        let active_model = Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            trigger: Set(create_data.trigger),
            response_id: Set(message.id),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.trigger {
            active_model.trigger = Set(value);
        }

        // Handle message_data update
        if let Some(message_data) = update_data.response_data {
            if let ActiveValue::Unchanged(message_id) = active_model.response_id {
                let _ = MessageQueries::update_by_id(db, message_id, message_data).await?;
            } else {
                let message = MessageQueries::create_entity(db, message_data).await?;
                active_model.response_id = Set(message.id);
            }
        }

        Ok(())
    }

    async fn delete_by_id<K>(db: &DatabaseConnection, id: K) -> Result<DeleteResult, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        let model = Self::Entity::find_by_id(id.into())
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("AutoRes not found"))?;

        let result = Self::Entity::delete_by_id(model.id).exec(db).await.map_err(AppError::from);

        // Delete related message
        // if let Some(message_id) = model.response_id {
        //     MessageQueries::delete_by_id(db, message_id).await?;
        // }

        result
    }
}
