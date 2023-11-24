use async_trait::async_trait;
use axum::{ Extension, Json, extract::Path, Router, routing::{ get, delete, post, patch } };
use sea_orm::{ EntityTrait, PrimaryKeyTrait, IntoActiveModel, DeleteResult };
use serde::{ Serialize, de::DeserializeOwned, Deserialize };

use crate::{
    default_queries::DefaultSeaQueries,
    app_state::AppState,
    utilities::app_error::AppError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataJson<T> where T: Serialize {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataList<T> where T: Serialize {
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataMessage {
    pub message: String,
}

type PrimaryKey = i32;

/// `DefaultRoutes` trait provides a standardized approach to defining web routes.
/// This trait is useful for applications that follow a consistent structure for their
/// CRUD operations, allowing for clean and maintainable code.
///
/// ### Methods
/// - `path`: Returns a string representing the base path for the routes associated with the entity.
///
/// ### Route Handlers
/// - `get_all`: Retrieves all instances of the entity as a JSON response.
/// - `get_one`: Retrieves a single instance of the entity based on its primary key as a JSON response.
/// - `create_one`: Creates a new instance of the entity from provided JSON data and returns it as a JSON response.
/// - `update_by_id`: Updates an existing entity based on its primary key and provided JSON data, returning the updated entity as a JSON response.
/// - `delete_by_id`: Deletes an entity based on its primary key and returns a confirmation message as a JSON response.
///
/// ### Router Method
/// - `default_router`: Generates a router with pre-defined routes for CRUD operations, mapping HTTP methods to the appropriate handlers using Axum's routing capabilities.
///
/// ## Usage
/// Implement `DefaultRoutes` for a struct representing routes for a specific entity to automatically generate standard CRUD operation routes.
#[async_trait]
pub trait DefaultRoutes: 'static {
    /// Represents the set of database operations associated with an entity.
    ///
    /// Must implement `DefaultSeaQueries`
    type Queries: DefaultSeaQueries + Send + Sync;

    /// The structure for the JSON response. Must be serializable, deserializable,
    /// and constructible from the associated model.
    type ResponseJson: Serialize +
        DeserializeOwned +
        Send +
        Sync +
        From<<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model>;

    /// Retrieves all instances of the associated entity from the database and
    /// returns them as a JSON response.
    ///
    /// This asynchronous function is a handler for a GET request to retrieve all
    /// records of the entity managed by this route. It uses the `find_all` method
    /// from the associated `Queries` to fetch all records from the database.
    ///
    /// ### Parameters
    /// - `Extension(state)`: An `Extension` extractor that provides access to the
    ///   shared application state, typically including a database connection.
    ///
    /// ### Returns
    /// - `Result<Json<ResponseList<Self::ResponseJson>>, AppError>`: On success,
    ///   returns a `Json` wrapper containing a `ResponseList` of `ResponseJson`
    ///   objects representing the fetched entities. On failure, returns an `AppError`.
    async fn get_all(Extension(state): Extension<AppState>) -> Result<
        Json<ResponseDataList<Self::ResponseJson>>,
        AppError
    > {
        let models: Vec<<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model> =
            Self::Queries::find_all(&state.db).await?;
        let response: Vec<Self::ResponseJson> = models
            .into_iter()
            .map(Self::ResponseJson::from)
            .collect();

        Ok(Json(ResponseDataList { data: response }))
    }

    /// Retrieves a single instance of the associated entity by its primary key and
    /// returns it as a JSON response.
    ///
    /// ### Parameters
    /// - `Extension(state)`: An `Extension` extractor providing access to the shared application state.
    /// - `Path(id)`: A `Path` extractor to capture the primary key from the request URL.
    ///
    /// ### Type Constraints
    /// - `PrimaryKey::ValueType`: The primary key's value type must be convertible from `i32`.
    ///
    /// ### Returns
    ///
    /// `Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>`
    ///
    /// - Ok(Json<ResponseDataJson<Self::ResponseJson>>): on success, returns a `Json` wrapper containing the `ResponseDataJson` with the fetched entity.
    /// - AppError: an AppError if the query request failure
    async fn get_one(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>
    ) -> Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>
    {
        let model: <<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::Queries::find_by_id(
            &state.db,
            id
        ).await?;
        let response: Self::ResponseJson = Self::ResponseJson::from(model);

        Ok(Json(ResponseDataJson { data: response }))
    }

    /// Creates a new instance of the associated entity in the database based on the provided JSON data
    /// and returns the created entity as a JSON response.
    ///
    /// ### Parameters
    /// - `Extension(state)`: An `Extension` extractor providing access to the shared application state.
    /// - `Json(create_dto)`: A `Json` extractor containing the data transfer object for entity creation.
    ///
    /// ### Returns
    ///
    /// `Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>`
    ///
    /// - Json<ResponseDataJson<Self::ResponseJson>>, on success, returns a `Json` wrapper containing the `ResponseDataJson` with the created entity.
    /// - AppError: on failure, returns an `AppError`.
    async fn create_one(
        Extension(state): Extension<AppState>,
        Json(create_dto): Json<<Self::Queries as DefaultSeaQueries>::CreateData>
    ) -> Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError> {
        let model: <<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::Queries::create_entity(
            &state.db,
            create_dto
        ).await?;

        let response: Self::ResponseJson = Self::ResponseJson::from(model);

        Ok(Json(ResponseDataJson { data: response }))
    }

    /// Updates an existing entity in the database identified by its primary key with the provided JSON data
    /// and returns the updated entity as a JSON response.
    ///
    /// ### Parameters
    /// - `Extension(state)`: An `Extension` extractor providing access to the shared application state.
    /// - `Path(id)`: A `Path` extractor to capture the primary key from the request URL.
    /// - `Json(update_dto)`: A `Json` extractor containing the data transfer object for entity update.
    ///
    /// ### Type Constraints
    /// - `PrimaryKey::ValueType`: The primary key's value type must be convertible from `i32`.
    /// - `Entity::Model`: Must be convertible into the associated active model.
    ///
    /// ### Returns
    ///
    /// `Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>`
    ///
    /// - Json<ResponseDataJson<Self::ResponseJson>: on success, returns a `Json` wrapper containing the `ResponseDataJson` with the updated entity.
    /// - AppError: on failure, returns an `AppError`.
    async fn update_by_id(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>,
        Json(update_dto): Json<<Self::Queries as DefaultSeaQueries>::UpdateData>
    )
        -> Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = Self::Queries::update_by_id(
            &state.db,
            id,
            update_dto
        ).await?;

        let response: Self::ResponseJson = Self::ResponseJson::from(model);

        Ok(Json(ResponseDataJson { data: response }))
    }

    /// Deletes an entity from the database by its primary key and returns a confirmation message as a JSON response.
    ///
    /// ### Parameters
    /// - `Extension(state)`: An `Extension` extractor providing access to the shared application state.
    /// - `Path(id)`: A `Path` extractor to capture the primary key from the request URL.
    ///
    /// ### Type Constraints
    /// - `PrimaryKey::ValueType`: The primary key's value type must be convertible from `i32`.
    ///
    /// ### Returns
    ///
    /// `Result<Json<ResponseMessage>, AppError>`
    ///
    /// - Json<ResponseMessage: on success, returns a `Json` wrapper containing a `ResponseMessage` with the details of the delete operation.
    /// - AppError: on failure, returns an `AppError`.
    async fn delete_by_id(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>
    ) -> Result<Json<ResponseDataMessage>, AppError>
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>
    {
        let result: DeleteResult = Self::Queries::delete_by_id(&state.db, id).await?;
        let message: String = format!("{} row(s) deleted", result.rows_affected);
        Ok(Json(ResponseDataMessage { message }))
    }

    /// Returns a string representing the base path for the routes associated with the entity.
    fn path() -> String;

    /// Constructs a default router for CRUD operations on a specific entity.
    /// This function generates routes for standard CRUD operations including list, retrieve,
    /// create, update, and delete, and associates them with their respective handler functions.
    ///
    /// ### Parameters
    /// - `state`: The shared application state, typically including a database connection and other shared resources.
    ///
    /// ### Returns
    /// Returns an `axum::Router` instance configured with the standard set of CRUD routes.
    ///
    /// ### Constraints
    /// The associated `Entity` type for the queries must have a primary key that can be converted from an `i32`.
    /// Additionally, the `Entity`'s model must be convertible into its corresponding active model.
    ///
    /// ### Routes
    /// - `GET /{path}`: Retrieves a list of all entities. Maps to `get_all`.
    /// - `GET /{path}/:id`: Retrieves a single entity by its ID. Maps to `get_one`.
    /// - `POST /{path}`: Creates a new entity from the provided JSON body. Maps to `create_one`.
    /// - `PATCH /{path}/:id`: Updates an existing entity by ID based on the provided JSON body. Maps to `update_by_id`.
    /// - `DELETE /{path}/:id`: Deletes an entity by its ID. Maps to `delete_by_id`.
    ///
    /// ### Example Usage
    /// This function is typically called within a specific route implementation to generate
    /// a router instance with all the necessary routes for an entity.
    ///
    /// ### Notes
    /// - The `AppState` type should be a shared type across the application, containing
    ///   global data such as database connections.
    /// - The `path` method of the implementing struct defines the base path for the routes.
    /// - Custom routes can be added to the router in addition to the default ones.
    ///
    /// ### Advantages
    /// - **Standardization**: Ensures a consistent routing structure across different parts of the application.
    /// - **Ease of Use**: Simplifies the creation of a fully functional router with minimal boilerplate.
    /// - **Flexibility**: Allows for easy extension and customization of routes.
    async fn default_router(state: AppState) -> Router
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let path = Self::path();
        Router::new()
            .route(&format!("/{}", &path), get(Self::get_all))
            .route(&format!("/{}/:id", &path), get(Self::get_one))
            .route(&format!("/{}", &path), post(Self::create_one))
            .route(&format!("/{}/:id", &path), patch(Self::update_by_id))
            .route(&format!("/{}/:id", &path), delete(Self::delete_by_id))
            .layer(Extension(state))
    }

    /// Provides a way for implementers to add more routes to the router.
    /// By default, this returns an empty router. Implementers can override
    /// this method to add custom routes.
    ///
    /// ### Returns
    /// - `Router`: A router with custom routes defined by the implementer.
    ///
    /// ### Example Usage
    /// ```rust,ignore
    /// struct MyCustomRoutes;
    ///
    /// #[async_trait]
    /// impl DefaultRoutes for MyCustomRoutes {
    ///     // ... other trait methods ...
    ///
    ///     async fn more_routes() -> Router {
    ///         Router::new()
    ///             .route("/custom", axum::routing::get(custom_handler))
    ///     }
    /// }
    ///
    /// async fn custom_handler() -> &'static str {
    ///     "This is a custom route"
    /// }
    /// ```
    #[allow(unused_variables)]
    async fn more_routes(state: AppState) -> Router
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        Router::new()
    }

    /// Creates the complete router combining the default routes and any additional
    /// custom routes provided by `more_routes`.
    ///
    /// ### Parameters
    /// - `state`: The shared application state, typically including a database connection and other shared resources.
    ///
    /// ### Returns
    /// - `Router`: A router combining the default CRUD routes with any additional custom routes.
    ///
    /// ### Example Usage
    /// ```rust,ignore
    /// let app_state = //... initialize AppState ...
    /// let router = MyCustomRoutes::router(app_state).await;
    /// ```
    async fn router(state: AppState) -> Router
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let default_routes = Self::default_router(state.clone()).await;
        let custom_routes = Self::more_routes(state.clone()).await;

        default_routes.merge(custom_routes)
    }
}
