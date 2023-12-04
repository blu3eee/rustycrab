use axum::{ Extension, Json, routing::{ get, patch }, Router };
use axum::extract::Path;
use rustycrab_model::response::{ ResponseDataJson, bots::ResponseBot };
use sea_orm::{ EntityTrait, PrimaryKeyTrait, IntoActiveModel };
use async_trait::async_trait;

use crate::app_state::AppState;
use crate::default_queries::DefaultSeaQueries;
use crate::utilities::app_error::AppError;
use crate::{ default_router::DefaultRoutes, queries::bot_queries::BotQueries };
use crate::database::bots::Model as BotModel;

pub struct BotsRouter {}

impl BotsRouter {
    async fn get_one_by_discord_id(
        Extension(state): Extension<AppState>,
        Path(bot_discord_id): Path<String>
    ) -> Result<Json<ResponseDataJson<ResponseBot>>, AppError> {
        let model = BotQueries::find_by_discord_id(&state.db, &bot_discord_id).await?;

        let response = ResponseBot::from(model);
        Ok(Json(ResponseDataJson { data: response }))
    }

    async fn update_by_discord_id(
        Extension(state): Extension<AppState>,
        Path(bot_discord_id): Path<String>,
        Json(
            update_dto,
        ): Json<<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::UpdateData>
    )
        -> Result<Json<ResponseDataJson<ResponseBot>>, AppError>
        where
            <<<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = <BotsRouter as DefaultRoutes>::Queries::update_by_discord_id(
            &state.db,
            &bot_discord_id,
            update_dto
        ).await?;

        let response: <BotsRouter as DefaultRoutes>::ResponseJson = <BotsRouter as DefaultRoutes>::ResponseJson::from(
            model
        );

        Ok(Json(ResponseDataJson { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for BotsRouter {
    type Queries = BotQueries;

    type ResponseJson = ResponseBot;

    fn path() -> String {
        format!("bots")
    }

    async fn more_routes() -> Router
        where
            <<<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<BotsRouter as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        Router::new().nest(
            &format!("/{}", &Self::path()),
            Router::new()
                .route("/discord/:bot_discord_id", get(Self::get_one_by_discord_id))
                .route("/discord/:bot_discord_id", patch(Self::update_by_discord_id))
        )
    }
}

impl From<BotModel> for ResponseBot {
    fn from(model: BotModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            token: model.token,
            theme_hex_color: model.theme_hex_color,
            discord_callback_url: model.discord_callback_url,
            discord_secret: model.discord_secret,
            premium_flags: model.premium_flags,
        }
    }
}
