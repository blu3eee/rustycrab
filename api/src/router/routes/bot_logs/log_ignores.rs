use crate::{
    database::{
        log_ignore_channels::Model as LogIgnoreChannelModel,
        log_ignore_roles::Model as LogIgnoreRoleModel,
    },
    default_router::{ DefaultRoutes, ResponseDataList },
    queries::guild_logs::{
        log_ignore_channel_queries::LogIgnoreChannelQueries,
        log_ignore_role_queries::LogIgnoreRoleQueries,
    },
    app_state::AppState,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};

use async_trait::async_trait;
use axum::{ Extension, extract::Path, Json, Router, routing::get };
use sea_orm::{ EntityTrait, IntoActiveModel, PrimaryKeyTrait };
use serde::{ Deserialize, Serialize };

pub async fn ignore_routes(state: AppState) -> Router {
    Router::new().nest(
        "/ignores",
        Router::new()
            .merge(BotGuildLogIgnoresChannelRoutes::router(state.clone()).await)
            .merge(BotGuildLogIgnoresRoleRoutes::router(state.clone()).await)
    )
}

pub struct BotGuildLogIgnoresChannelRoutes {}

impl BotGuildLogIgnoresChannelRoutes {
    pub async fn check_ignored(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id, channel_discord_id)): Path<(String, String, String)>
    ) -> Result<Json<bool>, AppError> {
        let ignored: bool = <Self as DefaultRoutes>::Queries::check_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &guild_discord_id,
            &channel_discord_id
        ).await?;

        Ok(Json(ignored))
    }

    pub async fn get_guild_ignores_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataList<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models = <Self as DefaultRoutes>::Queries::get_guild_ignores_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &guild_discord_id
        ).await?;

        let response: Vec<<Self as DefaultRoutes>::ResponseJson> = models
            .into_iter()
            .map(<Self as DefaultRoutes>::ResponseJson::from)
            .collect();

        Ok(Json(ResponseDataList { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for BotGuildLogIgnoresChannelRoutes {
    type Queries = LogIgnoreChannelQueries;

    type ResponseJson = ResponseLogIgnoreChannel;

    fn path() -> String {
        format!("channels")
    }

    async fn more_routes(_: AppState) -> Router {
        let path = Self::path();
        Router::new().route(
            &format!("/{}/:bot_discord_id/:guild_discord_id", &path),
            get(Self::get_guild_ignores_by_discord_ids)
        )
    }
}

pub struct BotGuildLogIgnoresRoleRoutes {}

impl BotGuildLogIgnoresRoleRoutes {}

impl BotGuildLogIgnoresRoleRoutes {
    pub async fn check_ignored(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id, role_discord_id)): Path<(String, String, String)>
    ) -> Result<Json<bool>, AppError> {
        let ignored: bool = <Self as DefaultRoutes>::Queries::check_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &guild_discord_id,
            &role_discord_id
        ).await?;

        Ok(Json(ignored))
    }
    pub async fn get_guild_ignores_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataList<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models = <Self as DefaultRoutes>::Queries::get_guild_ignores_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &guild_discord_id
        ).await?;

        let response: Vec<<Self as DefaultRoutes>::ResponseJson> = models
            .into_iter()
            .map(<Self as DefaultRoutes>::ResponseJson::from)
            .collect();

        Ok(Json(ResponseDataList { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for BotGuildLogIgnoresRoleRoutes {
    type Queries = LogIgnoreRoleQueries;

    type ResponseJson = ResponseLogIgnoreRole;

    fn path() -> String {
        format!("roles")
    }

    async fn more_routes(_: AppState) -> Router {
        let path = Self::path();
        Router::new().route(
            &format!("/{}/:bot_discord_id/:guild_discord_id", &path),
            get(Self::get_guild_ignores_by_discord_ids)
        )
    }
}

#[derive(Deserialize)]
pub struct RequestCreateLogIgnoreChannel {
    pub log_setting_id: i32,
    pub channel_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateLogIgnoreChannel {}

#[derive(Serialize, Deserialize)]
pub struct ResponseLogIgnoreChannel {
    pub id: i32,
    pub log_setting_id: Option<i32>,
    pub channel_id: String,
}

impl From<LogIgnoreChannelModel> for ResponseLogIgnoreChannel {
    fn from(model: LogIgnoreChannelModel) -> Self {
        Self {
            id: model.id,
            log_setting_id: model.log_setting_id,
            channel_id: model.channel_id,
        }
    }
}

#[derive(Deserialize)]
pub struct RequestCreateLogIgnoreRole {
    pub log_setting_id: i32,
    pub role_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateLogIgnoreRole {}

#[derive(Serialize, Deserialize)]
pub struct ResponseLogIgnoreRole {
    pub id: i32,
    pub log_setting_id: Option<i32>,
    pub role_id: String,
}

impl From<LogIgnoreRoleModel> for ResponseLogIgnoreRole {
    fn from(model: LogIgnoreRoleModel) -> Self {
        Self {
            id: model.id,
            log_setting_id: model.log_setting_id,
            role_id: model.role_id,
        }
    }
}
