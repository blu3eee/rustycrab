use async_trait::async_trait;
use axum::{ Extension, extract::Path, Router, Json, routing::{ get, patch } };
use rustycrab_model::response::{
    ticket::support_team::ResponseTicketSupportTeam,
    ResponseDataList,
    ResponseDataJson,
};
use sea_orm::{ EntityTrait, PrimaryKeyTrait, IntoActiveModel };
use crate::{
    database::ticket_support_teams::Model as TicketSupportTeamModel,
    queries::tickets_system::ticket_support_team_queries::TicketSupportTeamQueries,
    app_state::AppState,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
    default_router::DefaultRoutes,
};

impl From<TicketSupportTeamModel> for ResponseTicketSupportTeam {
    fn from(model: TicketSupportTeamModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            name: model.name,
            roles: model.roles.split(',').map(String::from).collect(),
            users: model.users.split(',').map(String::from).collect(),
        }
    }
}

pub struct TicketSupportTeamRoutes {}

impl TicketSupportTeamRoutes {
    pub async fn get_guild_support_teams_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataList<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models = <Self as DefaultRoutes>::Queries::find_guild_support_teams_by_discord_ids(
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

    pub async fn get_support_team_by_name(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id, name)): Path<(String, String, String)>
    )
        -> Result<Json<ResponseDataJson<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model = <Self as DefaultRoutes>::Queries::find_support_team_by_name(
            &state.db,
            &bot_discord_id,
            &guild_discord_id,
            &name
        ).await?;

        Ok(Json(ResponseDataJson { data: model.into() }))
    }

    pub async fn patch_support_team_by_name(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id, name)): Path<(String, String, String)>,
        Json(update_dto): Json<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::UpdateData>
    )
        -> Result<Json<ResponseDataJson<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model = <Self as DefaultRoutes>::Queries::update_support_team_by_name(
            &state.db,
            &bot_discord_id,
            &guild_discord_id,
            &name,
            update_dto
        ).await?;

        Ok(Json(ResponseDataJson { data: model.into() }))
    }
}

#[async_trait]
impl DefaultRoutes for TicketSupportTeamRoutes {
    type Queries = TicketSupportTeamQueries;
    type ResponseJson = ResponseTicketSupportTeam;

    fn path() -> String {
        format!("teams")
    }

    async fn more_routes(_: AppState) -> Router {
        let path = Self::path();
        Router::new()
            .route(
                &format!("/{}/:bot_discord_id/:guild_discord_id", &path),
                get(Self::get_guild_support_teams_by_discord_ids)
            )
            .route(
                &format!("/{}/:bot_discord_id/:guild_discord_id/:name", &path),
                get(Self::get_support_team_by_name)
            )
            .route(
                &format!("/{}/:bot_discord_id/:guild_discord_id/:name", &path),
                patch(Self::patch_support_team_by_name)
            )
    }
}
