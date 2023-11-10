use sea_orm::DatabaseConnection;

use sea_orm::{ entity::*, query::*, ActiveModelTrait, Set };

use crate::{ routes::users::RequestCreateUser, database::users::{ self, Model as UserModel } };

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

pub async fn get_one_user(
    db: &DatabaseConnection,
    id: &i32
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    users::Entity::find_by_id(*id).one(db).await
}

pub async fn get_one_user_by_discord_id(
    db: &DatabaseConnection,
    user_discord_id: &str
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    users::Entity::find().filter(users::Column::DiscordId.eq(user_discord_id)).one(db).await
}

// pub async fn update_user(db: &DatabaseConnection) {}
