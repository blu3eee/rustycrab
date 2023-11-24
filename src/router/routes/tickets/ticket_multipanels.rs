use async_trait::async_trait;
use axum::{ Extension, extract::Path, Json, Router, routing::get };
use sea_orm::{
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    JoinType::LeftJoin,
    RelationTrait,
    QuerySelect,
    ColumnTrait,
    PrimaryKeyTrait,
    IntoActiveModel,
};
use serde::{ Serialize, Deserialize };
use twilight_model::{ channel::message::{ Embed, component::ActionRow, Component }, id::Id };

use crate::{
    database::{
        ticket_multi_panels::Model as TicketMultiPanelModel,
        ticket_multi_panels_panels_ticket_panels::{
            self as PanelLink,
            Entity as PanelLinks,
            Relation as PanelLinksRelations,
        },
    },
    router::routes::{
        ResponseMessageDetails,
        bots::ResponseBot,
        guilds::ResponseGuild,
        RequestCreateUpdateMessage,
    },
    utilities::app_error::AppError,
    queries::{
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
        message_queries::MessageQueries,
        tickets_system::{
            ticket_panels_queries::TicketPanelsQueries,
            ticket_multipanels_queries::{ TicketMultiPanelQueries, create_button_components },
        },
        message_embed_queries::MessageEmbedQueries,
    },
    default_queries::DefaultSeaQueries,
    default_router::{ DefaultRoutes, ResponseDataList, ResponseDataMessage },
    app_state::AppState,
    twilightrs::messages::DiscordEmbed,
};

use super::ticket_panels::ResponseTicketPanel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketMultiPanel {
    pub id: i32,
    pub channel_id: String,
    pub sent_message_id: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub message_id: Option<i32>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketMultiPanelDetails {
    pub id: i32,
    pub channel_id: String,
    pub sent_message_id: String,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub message: Option<ResponseMessageDetails>,
    pub panels: Vec<ResponseTicketPanel>,
}

impl ResponseTicketMultiPanel {
    pub async fn to_details(
        &self,
        db: &DatabaseConnection
    ) -> Result<ResponseTicketMultiPanelDetails, AppError> {
        let bot: ResponseBot = BotQueries::find_by_id(db, self.bot_id).await?.into();
        let guild: ResponseGuild = GuildQueries::find_by_id(db, self.guild_id).await?.into();
        let message: Option<ResponseMessageDetails> = if let Some(id) = self.message_id {
            Some(MessageQueries::fetch_message_response(db, id).await?)
        } else {
            None
        };

        let panel_links = PanelLinks::find()
            .join(LeftJoin, PanelLinksRelations::TicketMultiPanels.def())
            .join(LeftJoin, PanelLinksRelations::TicketPanels.def())
            .filter(PanelLink::Column::TicketMultiPanelsId.eq(self.id))
            .all(db).await
            .map_err(AppError::from)?;

        let mut panels: Vec<ResponseTicketPanel> = Vec::new();
        for link in panel_links {
            let panel: ResponseTicketPanel = TicketPanelsQueries::find_by_id(
                db,
                link.ticket_panels_id
            ).await?.into();
            panels.push(panel.into());
        }

        Ok(ResponseTicketMultiPanelDetails {
            id: self.id,
            channel_id: self.channel_id.clone(),
            sent_message_id: self.sent_message_id.clone(),
            bot,
            guild,
            message,
            panels,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateTicketMultiPanel {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub channel_discord_id: String,
    pub message_data: RequestCreateUpdateMessage,
    pub panel_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateTicketMultiPanel {
    pub channel_discord_id: Option<String>,
    pub message_data: Option<RequestCreateUpdateMessage>,
    pub panel_ids: Option<Vec<i32>>,
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

        let message_id = panel.message_id.ok_or_else(|| {
            AppError::bad_request("Message ID not found")
        })?;

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
