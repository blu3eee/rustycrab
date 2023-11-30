use async_trait::async_trait;
use rustycrab_model::response::bot_users::{ RequestCreateBotUser, RequestUpdateBotUser };
use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    Condition,
    ActiveModelTrait,
    Set,
};

use crate::{
    utilities::app_error::AppError,
    database::{ bots, bot_users::{ Entity as BotUser, Model as BotUserModel, self }, users },
    default_queries::DefaultSeaQueries,
};

use super::{ bot_queries::BotQueries, user_queries::UserQueries };

pub struct BotUserQueries {}

impl BotUserQueries {
    pub async fn find_by_discord_ids(
        db: &DatabaseConnection,
        bot_id: &str,
        user_id: &str
    ) -> Result<BotUserModel, AppError> {
        BotUser::find()
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_id))
                    .add(users::Column::DiscordId.eq(user_id))
            )
            .one(db).await
            .map_err(AppError::from)
            .and_then(|bot_user| bot_user.ok_or_else(|| AppError::not_found("Bot User not found")))
    }

    pub async fn update_by_discord_ids(
        db: &DatabaseConnection,
        bot_id: &str,
        user_id: &str,
        update_data: RequestUpdateBotUser
    ) -> Result<BotUserModel, AppError> {
        let model = Self::find_by_discord_ids(db, bot_id, user_id).await?;
        let mut active_model: bot_users::ActiveModel = model.into();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }

    pub async fn get_one_or_create(
        db: &DatabaseConnection,
        bot_id: &str,
        user_id: &str
    ) -> Result<BotUserModel, AppError> {
        match Self::find_by_discord_ids(db, bot_id, user_id).await {
            Ok(bot_user) => Ok(bot_user),
            Err(_) =>
                Self::create_entity(db, RequestCreateBotUser {
                    bot_id: bot_id.to_owned(),
                    user_id: user_id.to_owned(),
                }).await,
        }
    }

    pub async fn get_all_bot_users(
        db: &DatabaseConnection,
        bot_id: &str
    ) -> Result<Vec<BotUserModel>, AppError> {
        BotUser::find()
            .filter(Condition::all().add(bots::Column::BotId.eq(bot_id)))
            .all(db).await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl DefaultSeaQueries for BotUserQueries {
    type Entity = bot_users::Entity;
    type ActiveModel = bot_users::ActiveModel;

    type CreateData = RequestCreateBotUser;
    type UpdateData = RequestUpdateBotUser;
    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.balance {
            active_model.balance = Set(value);
        }
        if let Some(value) = update_data.pray_points {
            active_model.pray_points = Set(value);
        }
        if let Some(value) = update_data.inventory {
            active_model.inventory = Set(value);
        }
        Ok(())
    }
    async fn create_entity(
        db: &DatabaseConnection,
        create_dto: Self::CreateData
    ) -> Result<BotUserModel, AppError> {
        if
            let Ok(Some(bot_user)) = BotUser::find()
                .filter(Condition::all().add(bots::Column::BotId.eq(&create_dto.bot_id)))
                .filter(Condition::all().add(users::Column::DiscordId.eq(&create_dto.user_id)))
                .one(db).await
        {
            return Ok(bot_user);
        }

        let bot = BotQueries::find_by_discord_id(db, &create_dto.bot_id).await?;
        let user = UserQueries::find_user_or_create(db, &create_dto.user_id).await?;

        let bot_user: bot_users::ActiveModel = bot_users::ActiveModel {
            bot_id: Set(bot.id),
            user_id: Set(user.id),
            ..Default::default()
        };

        bot_user.insert(db).await.map_err(AppError::from)
    }
}
