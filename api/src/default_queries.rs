use crate::utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error };
use async_trait::async_trait;
use axum::http::StatusCode;

use sea_orm::{
    ActiveModelTrait,
    TryIntoModel,
    IntoActiveModel,
    DatabaseConnection,
    ActiveModelBehavior,
    EntityTrait,
    PrimaryKeyTrait,
    DeleteResult,
};
use serde::de::DeserializeOwned;

#[async_trait]
pub trait DefaultSeaQueries {
    type Entity: EntityTrait;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> +
        ActiveModelBehavior +
        TryIntoModel<<Self::Entity as EntityTrait>::Model> +
        Send +
        Sync;

    type CreateDto: DeserializeOwned + Send + Sync;
    type UpdateDto: DeserializeOwned + Send + Sync;

    async fn find_all(
        db: &DatabaseConnection
    ) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, AppError> {
        Self::Entity::find().all(db).await.map_err(convert_seaorm_error)
    }

    async fn find_by_id<K>(
        db: &DatabaseConnection,
        id: K
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        Self::Entity::find_by_id(id.into())
            .one(db).await
            .map_err(convert_seaorm_error)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateDto
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>;

    fn apply_updates(
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateDto
    ) -> Result<(), AppError>;

    async fn update_by_id<K>(
        db: &DatabaseConnection,
        id: K,
        update_data: Self::UpdateDto
    )
        -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync,
            <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>
    {
        let model: <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::find_by_id(
            db,
            id
        ).await?;

        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into_active_model();

        Self::apply_updates(&mut active_model, update_data)?;

        Self::save_active_model(db, active_model).await
    }

    async fn save_active_model(
        db: &DatabaseConnection,
        active_model: Self::ActiveModel
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>
    {
        let saved_model: <Self as DefaultSeaQueries>::ActiveModel = active_model
            .save(db).await
            .map_err(|error| {
                eprintln!("Error saving active model: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving active model")
            })?;

        Self::convert_active_to_model(saved_model)
    }

    fn convert_active_to_model(
        active_model: Self::ActiveModel
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        active_model.try_into_model().map_err(|error| {
            eprintln!("Error converting active model to model: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })
    }

    async fn delete_by_id<K>(db: &DatabaseConnection, id: K) -> Result<DeleteResult, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        Ok(Self::Entity::delete_by_id(id.into()).exec(db).await.map_err(convert_seaorm_error)?)
    }
}
