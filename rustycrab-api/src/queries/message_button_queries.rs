use async_trait::async_trait;
use rustycrab_model::response::discord_message::{ RequestCreateButton, RequestUpdateButton };
use sea_orm::{ DatabaseConnection, EntityTrait, Set };

use crate::{
    database::buttons::{ Entity as Buttons, ActiveModel as ButtonActiveModel },
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};

pub struct MessageButtonQueries {}

#[async_trait]
impl DefaultSeaQueries for MessageButtonQueries {
    type Entity = Buttons;
    type ActiveModel = ButtonActiveModel;

    type CreateData = RequestCreateButton;
    type UpdateData = RequestUpdateButton;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let active_model: ButtonActiveModel = ButtonActiveModel {
            text: Set(create_data.text),
            emoji: Set(create_data.emoji),
            color: Set(create_data.color),
            ..Default::default()
        };
        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(text) = update_data.text {
            active_model.text = Set(text);
        }

        if let Some(emoji) = update_data.emoji {
            active_model.emoji = Set(emoji);
        }

        if let Some(color) = update_data.color {
            active_model.color = Set(color);
        }

        Ok(())
    }
}
