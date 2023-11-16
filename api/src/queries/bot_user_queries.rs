use async_trait::async_trait;
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
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
    database::{ bots, bot_users::{ Entity as BotUser, Model as BotUserModel, self }, users },
    routes::bot_users::{ RequestUpdateBotUser, RequestCreateBotUser },
    default_queries::DefaultSeaQueries,
};

use super::{ user_queries::get_user_or_create, bot_queries::BotQueries };

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
            .map_err(convert_seaorm_error)
            .and_then(|bot_user| bot_user.ok_or_else(|| AppError::not_found("Bot User not found")))
    }

    pub async fn update_bot_user(
        db: &DatabaseConnection,
        bot_id: &str,
        user_id: &str,
        update_data: RequestUpdateBotUser
    ) -> Result<BotUserModel, AppError> {
        let model = Self::find_by_discord_ids(db, bot_id, user_id).await?;
        let mut active_model: bot_users::ActiveModel = model.into();

        Self::apply_updates(&mut active_model, update_data)?;

        Self::save_active_model(db, active_model).await
    }

    pub async fn get_bot_user_or_create(
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
            .map_err(convert_seaorm_error)
    }
}

#[async_trait]
impl DefaultSeaQueries for BotUserQueries {
    type Entity = bot_users::Entity;
    type ActiveModel = bot_users::ActiveModel;

    type CreateDto = RequestCreateBotUser;
    type UpdateDto = RequestUpdateBotUser;
    fn apply_updates(
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateDto
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
        create_dto: Self::CreateDto
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
        let user = get_user_or_create(db, &create_dto.user_id).await?;

        let bot_user: bot_users::ActiveModel = bot_users::ActiveModel {
            bot_id: Set(Some(bot.id)),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        bot_user.insert(db).await.map_err(convert_seaorm_error)
    }
}
