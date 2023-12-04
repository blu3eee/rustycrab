use async_trait::async_trait;
use rustycrab_model::response::items::{ RequestCreateBotItem, RequestUpdateBotItem };
use sea_orm::{
    Set,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    ColumnTrait,
    Condition,
    QuerySelect,
    RelationTrait,
};

use crate::{
    default_queries::DefaultSeaQueries,
    database::{ items, bots },
    utilities::app_error::AppError,
};
use super::bot_queries::BotQueries;

pub struct BotItemQueries {}

impl BotItemQueries {
    pub async fn find_by_item_id(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        item_id: &str
    ) -> Result<items::Model, AppError> {
        <Self as DefaultSeaQueries>::Entity
            ::find()
            .join(sea_orm::JoinType::LeftJoin, items::Relation::Bots.def())
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(items::Column::ItemId.eq(item_id))
            )
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }
}

#[async_trait]
impl DefaultSeaQueries for BotItemQueries {
    type Entity = items::Entity;
    type ActiveModel = items::ActiveModel;

    type CreateData = RequestCreateBotItem;
    type UpdateData = RequestUpdateBotItem;

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.name {
            active_model.name = Set(value);
        }
        if let Some(value) = update_data.item_id {
            active_model.item_id = Set(value);
        }
        if let Some(value) = update_data.emoji {
            active_model.emoji = Set(Some(value));
        }
        if let Some(value) = update_data.value {
            active_model.value = Set(Some(value));
        }
        if let Some(value) = update_data.functions {
            active_model.functions = Set(value.join(","));
        }
        Ok(())
    }

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<items::Model, AppError> {
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;

        if
            let Some(item) = <Self::Entity as EntityTrait>
                ::find()
                .filter(items::Column::ItemId.eq(&create_data.item_id))
                .one(db).await?
        {
            return Ok(item);
        }

        let active_model = Self::ActiveModel {
            bot_id: Set(bot.id),
            item_id: Set(create_data.item_id),
            name: Set(create_data.name),
            functions: Set(String::new()),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }
}
