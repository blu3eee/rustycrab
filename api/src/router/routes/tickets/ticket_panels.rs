use async_trait::async_trait;
use axum::{ Extension, Json, extract::Path, Router, routing::get };
use sea_orm::{ EntityTrait, IntoActiveModel, PrimaryKeyTrait, DatabaseConnection };
use serde::{ Serialize, Deserialize };
use twilight_model::{
    id::Id,
    channel::message::{ Embed, Component, component::{ ActionRow, Button }, ReactionType },
};
use crate::{
    database::ticket_panels::Model as TicketPanelModel,
    router::routes::{
        RequestCreateUpdateMessage,
        RequestCreateButton,
        RequestUpdateButton,
        bots::ResponseBot,
        guilds::ResponseGuild,
        ResponseMessageDetails,
        ResponseButton,
    },
    default_router::{ DefaultRoutes, ResponseDataList, ResponseDataMessage },
    queries::{
        tickets_system::{
            ticket_panels_queries::TicketPanelsQueries,
            ticket_support_team_queries::TicketSupportTeamQueries,
        },
        message_queries::MessageQueries,
        guild_queries::GuildQueries,
        bot_queries::BotQueries,
        message_button_queries::MessageButtonQueries,
        message_embed_queries::MessageEmbedQueries,
    },
    utilities::{ app_error::AppError, utils::color_to_button_style },
    default_queries::DefaultSeaQueries,
    app_state::AppState,
    twilightrs::messages::DiscordEmbed,
};

use super::ticket_support_teams::ResponseTicketSupportTeam;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateTicketPanel {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub mention_on_open: Vec<String>,
    pub naming_scheme: String,
    pub channel_id: String,
    pub message_data: RequestCreateUpdateMessage,
    pub button_data: RequestCreateButton,
    pub welcome_message_data: RequestCreateUpdateMessage,
    pub support_team_id: i32,
    pub ticket_category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateTicketPanel {
    pub mention_on_open: Option<Vec<String>>,
    pub naming_scheme: Option<String>,
    pub channel_id: Option<String>,
    pub sent_message_id: Option<String>,
    pub message_data: Option<RequestCreateUpdateMessage>,
    pub button_data: Option<RequestUpdateButton>,
    pub welcome_message_data: Option<RequestCreateUpdateMessage>,
    pub support_team_id: Option<i32>,
    pub ticket_category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketPanel {
    pub id: i32,
    pub mention_on_open: Vec<String>,
    pub naming_scheme: String,
    pub channel_id: String,
    pub sent_message_id: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub message_id: Option<i32>,
    pub button_id: Option<i32>,
    pub welcome_message_id: Option<i32>,
    pub support_team_id: Option<i32>,
    pub ticket_category: String,
}

impl From<TicketPanelModel> for ResponseTicketPanel {
    fn from(model: TicketPanelModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            mention_on_open: model.mention_on_open.split(',').map(String::from).collect(),
            naming_scheme: model.naming_scheme,
            channel_id: model.channel_id,
            sent_message_id: model.sent_message_id,
            message_id: model.message_id,
            button_id: model.button_id,
            welcome_message_id: model.welcome_message_id,
            support_team_id: model.support_team_id,
            ticket_category: model.ticket_category,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketPanelDetails {
    pub id: i32,
    pub mention_on_open: Vec<String>,
    pub naming_scheme: String,
    pub channel_id: String,
    pub sent_message_id: String,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub message: Option<ResponseMessageDetails>,
    pub button: Option<ResponseButton>,
    pub welcome_message: Option<ResponseMessageDetails>,
    pub support_team: Option<ResponseTicketSupportTeam>,
    pub ticket_category: String,
}

impl ResponseTicketPanel {
    pub async fn to_details(
        &self,
        db: &DatabaseConnection
    ) -> Result<ResponseTicketPanelDetails, AppError> {
        let bot: ResponseBot = BotQueries::find_by_id(db, self.bot_id).await?.into();

        let guild: ResponseGuild = GuildQueries::find_by_id(db, self.guild_id).await?.into();

        let message: Option<ResponseMessageDetails> = if let Some(id) = self.message_id {
            Some(MessageQueries::fetch_message_response(db, id).await?)
        } else {
            None
        };

        let button: Option<ResponseButton> = if let Some(id) = self.button_id {
            Some(MessageButtonQueries::find_by_id(db, id).await?.into())
        } else {
            None
        };

        let welcome_message: Option<ResponseMessageDetails> = if
            let Some(id) = self.welcome_message_id
        {
            Some(MessageQueries::fetch_message_response(db, id).await?)
        } else {
            None
        };

        let support_team: Option<ResponseTicketSupportTeam> = if
            let Some(id) = self.support_team_id
        {
            Some(TicketSupportTeamQueries::find_by_id(db, id).await?.into())
        } else {
            None
        };

        Ok(ResponseTicketPanelDetails {
            id: self.id,
            bot,
            guild,
            message,
            button,
            welcome_message,
            mention_on_open: self.mention_on_open.clone(),
            naming_scheme: self.naming_scheme.clone(),
            channel_id: self.channel_id.clone(),
            sent_message_id: self.sent_message_id.clone(),
            support_team,
            ticket_category: self.ticket_category.clone(),
        })
    }
}

pub struct TicketPanelsRoutes {}

#[async_trait]
impl DefaultRoutes for TicketPanelsRoutes {
    type Queries = TicketPanelsQueries;
    type ResponseJson = ResponseTicketPanel;

    fn path() -> String {
        format!("panels")
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

impl TicketPanelsRoutes {
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
        let panel = TicketPanelsQueries::find_by_id(&state.db, id).await?;

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

        let button_id = panel.button_id.ok_or_else(|| {
            AppError::bad_request("Button ID not found")
        })?;
        let button = MessageButtonQueries::find_by_id(&state.db, button_id).await?;

        let _ = client.http
            .create_message(Id::new(channel_id))
            .embeds(&vec![Embed::from(embed)])
            .map_err(|_| AppError::internal_server_error("Failed to create message embed"))?
            .components(
                &vec![
                    Component::ActionRow(ActionRow {
                        components: Vec::from([
                            Component::Button(Button {
                                custom_id: Some(format!("1:1:{}", panel.id)),
                                disabled: false,
                                emoji: if button.emoji.len() > 5 {
                                    let emoji_id = u64
                                        ::from_str_radix(&button.emoji, 10)
                                        .map_err(|_| {
                                            AppError::bad_request("Invalid emoji ID")
                                        })?;
                                    Some(ReactionType::Custom {
                                        animated: false,
                                        id: Id::new(emoji_id),
                                        name: None,
                                    })
                                } else {
                                    Some(ReactionType::Unicode { name: button.emoji })
                                },
                                label: Some(format!("{}", button.text)),
                                style: color_to_button_style(&button.color),
                                url: None,
                            }),
                        ]),
                    })
                ]
            )
            .map_err(|_| AppError::internal_server_error("Failed to create message button"))?.await
            .map_err(|_| AppError::bad_request("Failed to send message"))?;

        Ok(Json(ResponseDataMessage { message: "Panel sent".to_string() }))
    }
}
