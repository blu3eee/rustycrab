use async_trait::async_trait;
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, Condition, ColumnTrait, Set };

use crate::{
    database::marriages,
    default_queries::DefaultSeaQueries,
    utilities::app_error::AppError,
};

use rustycrab_model::response::marriages::{ RequestCreateMarriage, RequestUpdateMarriage };
use super::bot_user_queries::BotUserQueries;
pub struct MarriageQueries {}

#[async_trait]
impl DefaultSeaQueries for MarriageQueries {
    type Entity = marriages::Entity;

    /// The active model type associated with the entity. This is used
    /// for create and update operations.
    type ActiveModel = marriages::ActiveModel;

    /// Data transfer object (DTO) type for creating entities.
    type CreateData = RequestCreateMarriage;
    type UpdateData = RequestUpdateMarriage;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let botuser1 = BotUserQueries::find_or_create_by_discord_ids(
            db,
            &create_data.bot_discord_id,
            &create_data.user1_discord_id
        ).await?;
        let botuser2 = BotUserQueries::find_or_create_by_discord_ids(
            db,
            &create_data.bot_discord_id,
            &create_data.user2_discord_id
        ).await?;
        if
            let Some(_) = <Self::Entity as EntityTrait>
                ::find()
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(marriages::Column::Partner1Id.eq(botuser1.id))
                                .add(marriages::Column::Partner2Id.ne(botuser2.id))
                        )
                        .add(
                            Condition::all()
                                .add(marriages::Column::Partner1Id.eq(botuser2.id))
                                .add(marriages::Column::Partner2Id.ne(botuser1.id))
                        )
                )
                .one(db).await?
        {
            return Err(
                AppError::bad_request(
                    "bad request, one of the users already have marriage relationship"
                )
            );
        }

        if
            let Some(rela) = <Self::Entity as EntityTrait>
                ::find()
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(marriages::Column::Partner1Id.eq(botuser1.id))
                                .add(marriages::Column::Partner2Id.eq(botuser2.id))
                        )
                        .add(
                            Condition::all()
                                .add(marriages::Column::Partner1Id.eq(botuser2.id))
                                .add(marriages::Column::Partner2Id.eq(botuser1.id))
                        )
                )
                .one(db).await?
        {
            return Ok(rela);
        }

        let active_model = Self::ActiveModel {
            date_of_marriage: Set(create_data.date_of_marriage),
            partner1_id: Set(botuser1.id),
            partner2_id: Set(botuser2.id),
            // ring_id: todo!(),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.ring_id {
            active_model.ring_id = Set(Some(value));
        }
        if let Some(value) = update_data.date_of_marriage {
            active_model.date_of_marriage = Set(value);
        }
        if let Some(value) = update_data.thumbnail {
            active_model.thumbnail = Set(Some(value));
        }
        if let Some(value) = update_data.image {
            active_model.image = Set(Some(value));
        }
        if let Some(value) = update_data.quote {
            active_model.quote = Set(Some(value));
        }
        if let Some(value) = update_data.caption {
            active_model.caption = Set(Some(value));
        }
        Ok(())
    }
}
