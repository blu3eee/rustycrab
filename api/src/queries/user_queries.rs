use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set };

use crate::utilities::app_error::AppError;
use crate::{
    routes::users::RequestCreateUser,
    database::users::{ self, Entity as User, Model as UserModel },
};

// Assuming CreateUserDto exists and has the necessary fields
pub async fn create_user(
    db: &DatabaseConnection,
    dto: RequestCreateUser // Define this DTO to match the required input fields
) -> Result<UserModel, sea_orm::DbErr> {
    let active_model = users::ActiveModel {
        discord_id: Set(dto.discord_id),
        ..Default::default() // Use default values for other fields
    };

    active_model.insert(db).await
}

pub async fn get_user(
    db: &DatabaseConnection,
    id: &i32
) -> Result<Option<UserModel>, sea_orm::DbErr> {
    users::Entity::find_by_id(*id).one(db).await
}

pub async fn get_user_by_discord_id(
    db: &DatabaseConnection,
    user_discord_id: &str
) -> Result<Option<UserModel>, sea_orm::DbErr> {
    users::Entity::find().filter(users::Column::DiscordId.eq(user_discord_id)).one(db).await
}

pub async fn get_user_or_create(
    db: &DatabaseConnection,
    user_discord_id: &str
) -> Result<UserModel, AppError> {
    match User::find().filter(users::Column::DiscordId.eq(user_discord_id)).one(db).await {
        Ok(Some(guild)) => {
            // If the guild is found, return it
            Ok(guild)
        }
        Ok(None) => {
            // If the guild is not found, create a new one
            let new_guild = users::ActiveModel {
                discord_id: Set(user_discord_id.to_owned()),
                // Set other fields as needed, for example:
                // ... other fields ...
                ..Default::default() // Use default values for the rest of the fields
            };

            new_guild.insert(db).await.map_err(|err| {
                eprintln!("Error creating new user: {:?}", err);
                AppError::internal_server_error("There was an error creating the user")
            })
        }
        Err(err) => {
            // If there's an error querying the database, return an error
            eprintln!("Error getting user from discord id: {:?}", err);
            Err(AppError::internal_server_error("There was an error getting the user"))
        }
    }
}
// pub async fn update_user(db: &DatabaseConnection) {}
