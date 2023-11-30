use serde::{ Serialize, Deserialize };

use super::super::{
    discord_message::{ ResponseMessageDetails, RequestCreateUpdateMessage },
    bots::ResponseBot,
    guilds::ResponseGuild,
};
use super::panel::ResponseTicketPanel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketMultiPanel {
    pub id: i32,
    pub bot_id: i32,
    pub guild_id: i32,
    pub channel_id: String,
    pub message_id: i32,
    pub sent_message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketMultiPanelDetails {
    pub id: i32,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub message: ResponseMessageDetails,
    pub panels: Vec<ResponseTicketPanel>,
    pub channel_id: String,
    pub sent_message_id: String,
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
