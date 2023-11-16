use actix_web::http::StatusCode;
use axum::{ extract::Extension, Json };

use crate::{
    app_state::AppState,
    queries::guild_config_queries::GuildConfigQueries,
    utilities::app_error::AppError,
    routes::guild_configs::{ ResponseGuildConfig, ResponseDataGuildConfig },
    default_queries::DefaultSeaQueries,
};

use super::RequestCreateConfig;

pub async fn create_config(
    Extension(state): Extension<AppState>,
    Json(create_dto): Json<RequestCreateConfig>
) -> Result<(StatusCode, Json<ResponseDataGuildConfig>), AppError> {
    let config = GuildConfigQueries::create_entity(&state.db, create_dto).await?;
    let response: ResponseGuildConfig = config.into();

    Ok((StatusCode::CREATED, Json(ResponseDataGuildConfig { data: response })))
}
