use crate::utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error };
use async_trait::async_trait;

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

/// `DefaultSeaQueries` is a trait providing a set of default CRUD operations
/// for entities in a SeaORM context. It is designed to be implemented by
/// specific query structs that correspond to particular entities, enabling
/// standardized database interactions.
#[async_trait]
pub trait DefaultSeaQueries {
    /// The entity type associated with this set of queries.
    type Entity: EntityTrait;

    /// The active model type associated with the entity. This is used
    /// for create and update operations.
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> +
        ActiveModelBehavior +
        TryIntoModel<<Self::Entity as EntityTrait>::Model> +
        Send +
        Sync;

    /// Data transfer object (DTO) type for creating entities.
    type CreateData: DeserializeOwned + Send + Sync;
    /// Data transfer object (DTO) type for updating entities.
    type UpdateData: DeserializeOwned + Send + Sync;

    /// Retrieves all instances of the entity from the database.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    ///
    /// ### Returns
    /// - `Ok(Vec<Entity::Model>)`: A vector of entity models if successful.
    /// - `Err(AppError)`: An error if the query fails.
    async fn find_all(
        db: &DatabaseConnection
    ) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, AppError> {
        Self::Entity::find().all(db).await.map_err(convert_seaorm_error)
    }

    /// Finds a single instance of the entity by its primary key.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `id`: The primary key value of the entity to find.
    ///
    /// ### Returns
    /// - `Ok(Entity::Model)`: The found entity model if successful.
    /// - `Err(AppError)`: An error if the entity is not found or the query fails.
    async fn find_by_id(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where <<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>
    {
        Self::Entity::find_by_id(id)
            .one(db).await
            .map_err(convert_seaorm_error)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }

    /// Creates a new entity instance in the database based on the provided DTO data.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `create_data`: The DTO data used to create a new entity.
    ///
    /// ### Returns
    /// - `Ok(Entity::Model)`: The newly created entity model if successful.
    /// - `Err(AppError)`: An error if the creation fails.
    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError>;

    /// Applies updates to an active model instance based on the provided DTO data.
    ///
    /// ### Parameters
    /// - `active_model`: The mutable reference to the active model to be updated.
    /// - `update_data`: The DTO data containing the updates.
    ///
    /// ### Returns
    /// - `Ok(())`: If updates are applied successfully.
    /// - `Err(AppError)`: An error if the update fails.
    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError>;

    /// Updates an entity in the database by its primary key with the provided DTO data.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `id`: The primary key value of the entity to update.
    /// - `update_data`: The DTO data containing the updates.
    ///
    /// ### Returns
    /// - `Ok(Entity::Model)`: The updated entity model if successful.
    /// - `Err(AppError)`: An error if the update fails.
    async fn update_by_id(
        db: &DatabaseConnection,
        id: i32,
        update_data: Self::UpdateData
    )
        -> Result<<Self::Entity as EntityTrait>::Model, AppError>
        where
            <<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>
    {
        let model: <<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::find_by_id(
            db,
            id
        ).await?;

        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into_active_model();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }

    /// Deletes an entity from the database by its primary key.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `id`: The primary key value of the entity to delete.
    ///
    /// ### Returns
    /// - `Ok(DeleteResult)`: The result of the delete operation.
    /// - `Err(AppError)`: An error if the delete operation fails.
    async fn delete_by_id<K>(db: &DatabaseConnection, id: K) -> Result<DeleteResult, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        Ok(Self::Entity::delete_by_id(id.into()).exec(db).await.map_err(convert_seaorm_error)?)
    }

    /// Saves changes in an active model to the database and returns the updated model.
    ///
    /// ### Parameters
    /// - `db`: The database connection.
    /// - `active_model`: The active model with changes to be saved.
    ///
    /// ### Returns
    /// - `Ok(Entity::Model)`: The updated entity model if successful.
    /// - `Err(AppError)`: An error if the save operation fails.
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
                AppError::internal_server_error("Error saving active model")
            })?;

        Self::convert_active_to_model(saved_model)
    }

    /// Converts an active model instance to its corresponding entity model.
    ///
    /// ### Parameters
    /// - `active_model`: The active model to be converted.
    ///
    /// ### Returns
    /// - `Ok(Entity::Model)`: The corresponding entity model.
    /// - `Err(AppError)`: An error if the conversion fails.
    fn convert_active_to_model(
        active_model: Self::ActiveModel
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        active_model.try_into_model().map_err(|error| {
            eprintln!("Error converting active model to model: {:?}", error);
            AppError::internal_server_error("Internal server error")
        })
    }
}
