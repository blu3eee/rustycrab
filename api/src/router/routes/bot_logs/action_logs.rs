use crate::{
    database::guild_action_logs::Model as ActionLogModel,
    default_router::{ DefaultRoutes, ResponseDataJson, ResponseDataList },
    queries::guild_logs::action_log_queries::ActionLogsQueries,
    app_state::AppState,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};
use async_trait::async_trait;
use axum::{ Extension, extract::Path, Json, Router, routing::get };
use sea_orm::{ PrimaryKeyTrait, EntityTrait, IntoActiveModel };
use serde::{ Deserialize, Serialize };

pub struct ActionLogsRoutes {}

impl ActionLogsRoutes {
    pub async fn get_guild_actions_logs(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<
            Json<ResponseDataList<<ActionLogsRoutes as DefaultRoutes>::ResponseJson>>,
            AppError
        >
        where
            <<<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models = <ActionLogsRoutes as DefaultRoutes>::Queries::find_guild_action_logs(
            &state.db,
            &bot_discord_id,
            &guild_discord_id
        ).await?;
        let response: Vec<<ActionLogsRoutes as DefaultRoutes>::ResponseJson> = models
            .into_iter()
            .map(<ActionLogsRoutes as DefaultRoutes>::ResponseJson::from)
            .collect();

        Ok(Json(ResponseDataList { data: response }))
    }

    async fn get_unique(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id, channel_discord_id)): Path<(String, String, String)>
    )
        -> Result<
            Json<ResponseDataJson<<ActionLogsRoutes as DefaultRoutes>::ResponseJson>>,
            AppError
        >
        where
            <<<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<<ActionLogsRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = <ActionLogsRoutes as DefaultRoutes>::Queries::find_unique(
            &state.db,
            &bot_discord_id,
            &guild_discord_id,
            &channel_discord_id
        ).await?;
        let response: <ActionLogsRoutes as DefaultRoutes>::ResponseJson = <ActionLogsRoutes as DefaultRoutes>::ResponseJson::from(
            model
        );

        Ok(Json(ResponseDataJson { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for ActionLogsRoutes {
    type Queries = ActionLogsQueries;

    type ResponseJson = ResponseActionLog;

    fn path() -> String {
        format!("action-logs")
    }

    async fn more_routes(_: AppState) -> Router {
        Router::new()
            .route(
                &format!(
                    "/{}/:bot_discord_id/:guild_discord_id/:channel_discord_id",
                    &Self::path()
                ),
                get(Self::get_unique)
            )
            .route(
                &format!("/{}/:bot_discord_id/:guild_discord_id", &Self::path()),
                get(Self::get_guild_actions_logs)
            )
    }
}

#[derive(Deserialize)]
pub struct RequestCreateLogAction {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub channel_id: String,
    pub events: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateActionLog {
    pub channel_id: Option<String>,
    pub events: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseActionLog {
    pub id: i32,
    pub channel_id: String,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
    pub events: String,
}

impl From<ActionLogModel> for ResponseActionLog {
    fn from(model: ActionLogModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            channel_id: model.channel_id,
            events: model.events,
        }
    }
}
