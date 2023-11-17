use async_trait::async_trait;
use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set };

use crate::default_queries::DefaultSeaQueries;
use crate::routes::users_routes::{ RequestCreateUser, RequestUpdateUser };
use crate::utilities::app_error::AppError;
use crate::utilities::convert_seaorm_error::convert_seaorm_error;
use crate::database::users::{
    self,
    Entity as Users,
    Model as UserModel,
    ActiveModel as UserActiveModel,
};

use super::save_active_model;

pub struct UserQueries {}

impl UserQueries {
    pub async fn find_by_discord_id(
        db: &DatabaseConnection,
        user_discord_id: &str
    ) -> Result<UserModel, AppError> {
        Users::find()
            .filter(users::Column::DiscordId.eq(user_discord_id))
            .one(db).await
            .map_err(convert_seaorm_error)?
            .ok_or_else(|| AppError::not_found("User not found"))
    }

    pub async fn find_user_or_create(
        db: &DatabaseConnection,
        user_discord_id: &str
    ) -> Result<UserModel, AppError> {
        match Self::find_by_discord_id(db, user_discord_id).await {
            Ok(model) => {
                // If the guild is found, return it
                Ok(model)
            }
            Err(_) => {
                Self::create_entity(db, RequestCreateUser {
                    discord_id: user_discord_id.to_string(),
                }).await
            }
        }
    }

    pub async fn update_by_discord_id(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        update_data: RequestUpdateUser
    ) -> Result<UserModel, AppError> {
        // Fetch the bot by bot_id to update
        let model = Self::find_by_discord_id(db, bot_discord_id).await?;
        let mut active_model: users::ActiveModel = model.into();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }
}

#[async_trait]
impl DefaultSeaQueries for UserQueries {
    type Entity = Users;
    type ActiveModel = UserActiveModel;

    type CreateData = RequestCreateUser;
    type UpdateData = RequestUpdateUser;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if let Ok(user) = Self::find_by_discord_id(db, &create_data.discord_id).await {
            Ok(user)
        } else {
            save_active_model(db, UserActiveModel {
                discord_id: Set(create_data.discord_id),
                ..Default::default() // Use default values for other fields
            }).await
        }
    }

    #[allow(unused_variables)]
    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        // Apply updates from the DTO

        Ok(())
    }
}
