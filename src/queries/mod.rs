pub mod bot_queries;
pub mod guild_config_queries;
pub mod guild_queries;
pub mod user_queries;
pub mod bot_user_queries;
pub mod guild_welcome_queries;
pub mod message_button_queries;
pub mod message_embed_queries;
pub mod message_queries;
pub mod guild_logs;
pub mod tickets_system;
pub mod auto_responses_queries;

use crate::utilities::app_error::AppError;
use axum::http::StatusCode;

use sea_orm::{
    ActiveModelTrait,
    TryIntoModel,
    ModelTrait,
    IntoActiveModel,
    DatabaseConnection,
    ActiveModelBehavior,
    EntityTrait,
};

pub async fn save_active_model<A, M>(
    db: &DatabaseConnection,
    active_model: A
)
    -> Result<M, AppError>
    where
        A: ActiveModelTrait + ActiveModelBehavior + TryIntoModel<M> + Send, // Added `Send` bound here
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        M: ModelTrait + IntoActiveModel<A>
{
    let active_model = active_model.save(db).await.map_err(|error| {
        eprintln!("Error saving active model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving active model")
    })?;

    convert_active_to_model(active_model)
}

pub fn convert_active_to_model<A, M>(active_model: A) -> Result<M, AppError>
    where
        A: ActiveModelTrait + ActiveModelBehavior + TryIntoModel<M>,
        M: ModelTrait + IntoActiveModel<A>
{
    active_model.try_into_model().map_err(|error| {
        eprintln!("Error converting active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
