use async_trait::async_trait;
use axum::{ Extension, extract::Path, Json, Router, routing::{ get, patch } };
use sea_orm::{ EntityTrait, IntoActiveModel, PrimaryKeyTrait };
use serde::{ Deserialize, Serialize };

use crate::{
    database::users::Model as UserModel,
    default_router::{ DefaultRoutes, ResponseDataJson },
    queries::user_queries::UserQueries,
    app_state::AppState,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};

pub struct UsersRoutes {}

impl UsersRoutes {
    async fn get_one_by_discord_id(
        Extension(state): Extension<AppState>,
        Path(bot_discord_id): Path<String>
    ) -> Result<Json<ResponseDataJson<ResponseUser>>, AppError> {
        let model = UserQueries::find_by_discord_id(&state.db, &bot_discord_id).await?;

        let response = ResponseUser::from(model);

        Ok(Json(ResponseDataJson { data: response }))
    }

    async fn update_by_discord_id(
        Extension(state): Extension<AppState>,
        Path(bot_discord_id): Path<String>,
        Json(
            update_dto,
        ): Json<<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::UpdateData>
    )
        -> Result<Json<ResponseDataJson<ResponseUser>>, AppError>
        where
            <<<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = <UsersRoutes as DefaultRoutes>::Queries::update_by_discord_id(
            &state.db,
            &bot_discord_id,
            update_dto
        ).await?;

        let response: <UsersRoutes as DefaultRoutes>::ResponseJson = <UsersRoutes as DefaultRoutes>::ResponseJson::from(
            model
        );

        Ok(Json(ResponseDataJson { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for UsersRoutes {
    type Queries = UserQueries;

    type ResponseJson = ResponseUser;

    fn path() -> String {
        format!("users")
    }

    async fn more_routes(state: AppState) -> Router
        where
            <<<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<UsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let path = Self::path();
        Router::new()
            .route(&format!("/{}/discord/:bot_discord_id", &path), get(Self::get_one_by_discord_id))
            .route(
                &format!("/{}/discord/:bot_discord_id", &path),
                patch(Self::update_by_discord_id)
            )
            .layer(Extension(state))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateUser {
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateUser {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: i32,
    pub discord_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
impl From<UserModel> for ResponseUser {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            discord_id: model.discord_id,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
        }
    }
}
