use serde::{ Serialize, Deserialize };

use crate::response::{
    discord_message::{
        RequestCreateUpdateMessage,
        RequestCreateButton,
        ResponseMessageDetails,
        ResponseButton,
        RequestUpdateButton,
    },
    bots::ResponseBot,
    guilds::ResponseGuild,
};

use super::support_team::ResponseTicketSupportTeam;

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
    pub bot_id: i32,
    pub guild_id: i32,
    pub message_id: i32,
    pub welcome_message_id: i32,
    pub mention_on_open: Vec<String>,
    pub naming_scheme: String,
    pub channel_id: String,
    pub sent_message_id: String,
    pub button_id: i32,
    pub ticket_category: String,
    pub support_team_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketPanelDetails {
    pub id: i32,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub message: ResponseMessageDetails,
    pub welcome_message: ResponseMessageDetails,
    pub button: ResponseButton,
    pub support_team: Option<ResponseTicketSupportTeam>,
    pub mention_on_open: Vec<String>,
    pub sent_message_id: String,
    pub naming_scheme: String,
    pub channel_id: String,
    pub ticket_category: String,
}
