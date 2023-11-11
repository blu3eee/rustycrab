use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set };

use crate::{
    database::buttons::{
        self,
        Entity as Buttons,
        Model as ButtonModel,
        ActiveModel as ButtonActiveModel,
    },
    routes::{ RequestCreateButton, RequestUpdateButton },
    utilities::{ convert_seaorm_error::convert_seaorm_error, app_error::AppError },
};

pub async fn create_button(
    db: &DatabaseConnection,
    create_dto: RequestCreateButton
) -> Result<ButtonModel, AppError> {
    let new_button: ButtonActiveModel = ButtonActiveModel {
        text: Set(create_dto.text),
        emoji: Set(create_dto.emoji),
        color: Set(create_dto.color),
        ..Default::default()
    };

    new_button.insert(db).await.map_err(convert_seaorm_error)
}

pub async fn find_button(db: &DatabaseConnection, id: &i32) -> Result<ButtonModel, AppError> {
    Buttons::find()
        .filter(buttons::Column::Id.eq(*id))
        .one(db).await
        .map_err(|err| {
            eprintln!("Error getting bot with id: {:?}", err);
            AppError::internal_server_error("There was an error getting the button")
        })
        .and_then(|bot| bot.ok_or_else(|| AppError::not_found("Button not found")))
}

pub async fn update_button(
    db: &DatabaseConnection,
    id: &i32,
    update_dto: RequestUpdateButton
) -> Result<ButtonModel, AppError> {
    let mut active_button: ButtonActiveModel = find_button(db, id).await?.into();

    if let Some(text) = update_dto.text {
        active_button.text = Set(text);
    }

    if let Some(emoji) = update_dto.emoji {
        active_button.emoji = Set(emoji);
    }

    if let Some(color) = update_dto.color {
        active_button.color = Set(color);
    }

    active_button.update(db).await.map_err(convert_seaorm_error)
}

pub async fn delete_button(db: &DatabaseConnection, id: &i32) -> Result<(), AppError> {
    Buttons::delete_by_id(*id)
        .exec(db).await
        .map_err(|err| {
            eprintln!("Error deleting button with id {}: {:?}", id, err);
            AppError::internal_server_error("There was an error deleting the button")
        })?;

    Ok(())
}
