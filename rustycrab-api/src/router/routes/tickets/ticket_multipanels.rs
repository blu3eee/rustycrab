use async_trait::async_trait;
use axum::{ Extension, extract::Path, Json, Router, routing::get };
use rustycrab_model::response::{
    ticket::multipanel::ResponseTicketMultiPanel,
    ResponseDataList,
    ResponseDataMessage,
};
use sea_orm::{ EntityTrait, PrimaryKeyTrait, IntoActiveModel };
use twilight_model::{ channel::message::{ Embed, component::ActionRow, Component }, id::Id };

use crate::{
    database::ticket_multi_panels::Model as TicketMultiPanelModel,
    utilities::app_error::AppError,
    queries::{
        bot_queries::BotQueries,
        message_queries::MessageQueries,
        tickets_system::ticket_multipanels_queries::{
            TicketMultiPanelQueries,
            create_button_components,
        },
        message_embed_queries::MessageEmbedQueries,
    },
    default_queries::DefaultSeaQueries,
    app_state::AppState,
    twilightrs::messages::DiscordEmbed,
    default_router::DefaultRoutes,
};

impl From<TicketMultiPanelModel> for ResponseTicketMultiPanel {
    fn from(model: TicketMultiPanelModel) -> Self {
        Self {
            id: model.id,
            channel_id: model.channel_id,
            sent_message_id: model.sent_message_id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            message_id: model.message_id,
        }
    }
}

pub struct TicketMultiPanelsRoutes {}

#[async_trait]
impl DefaultRoutes for TicketMultiPanelsRoutes {
    type Queries = TicketMultiPanelQueries;

    type ResponseJson = ResponseTicketMultiPanel;

    fn path() -> String {
        format!("multipanels")
    }

    async fn more_routes(_: AppState) -> Router {
        let path = Self::path();
        Router::new()
            .route(
                &format!("/{}/:bot_discord_id/:guild_discord_id", &path),
                get(Self::get_panels_by_discord_ids)
            )
            .route(&format!("/{}/:id/send", &path), get(Self::send_panel))
    }
}

impl TicketMultiPanelsRoutes {
    pub async fn get_panels_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataList<<Self as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model: IntoActiveModel<<<Self as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models = <Self as DefaultRoutes>::Queries::find_panels_by_discord_ids(
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

    pub async fn send_panel(
        Extension(state): Extension<AppState>,
        Path(id): Path<i32>
    ) -> Result<Json<ResponseDataMessage>, AppError> {
        let panel = TicketMultiPanelQueries::find_by_id(&state.db, id).await?;

        let bot = BotQueries::find_by_id(&state.db, panel.bot_id).await?;
        let client = state.running_bots
            .get(&bot.bot_id)
            .ok_or_else(|| { AppError::not_found("Bot client not found") })?;

        let channel_id = u64
            ::from_str_radix(&panel.channel_id, 10)
            .map_err(|_| { AppError::bad_request("Invalid channel ID") })?;

        let message_id = panel.message_id;

        let message = MessageQueries::find_by_id(&state.db, message_id).await?;
        let embed_id = message.embed_id.ok_or_else(|| {
            AppError::bad_request("Embed ID not found")
        })?;

        let embed: DiscordEmbed = MessageEmbedQueries::find_by_id(
            &state.db,
            embed_id
        ).await?.into();

        let button_components = create_button_components(&state.db, panel.id).await?;
        println!("{:?}", button_components);
        let _ = client.http
            .create_message(Id::new(channel_id))
            .embeds(&vec![Embed::from(embed)])
            .map_err(|_| AppError::internal_server_error("Failed to create message embed"))?
            .components(
                &vec![
                    Component::ActionRow(ActionRow {
                        components: button_components,
                    })
                ]
            )
            .map_err(|_| AppError::internal_server_error("Failed to create message button"))?.await
            .map_err(|_| AppError::bad_request("Failed to send message"))?;

        Ok(Json(ResponseDataMessage { message: "Panel sent".to_string() }))
    }
}
