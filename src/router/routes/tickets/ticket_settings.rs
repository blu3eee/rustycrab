use async_trait::async_trait;
use serde::{ Serialize, Deserialize };
use crate::{
    database::ticket_settings::Model as TicketSettingModel,
    default_router::DefaultRoutes,
    queries::tickets_system::ticket_setting_queries::TicketSettingQueries,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateTicketSetting {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub per_user_ticket_limit: Option<i32>,
    pub allow_user_to_close_tickets: Option<bool>,
    pub ticket_close_confirmation: Option<bool>,
    pub ticket_notification_channel: Option<String>,
    pub transcripts_channel: Option<String>,
    pub thread_ticket: Option<bool>,
    pub archive_category: Option<String>,
    pub archive_overflow_category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateTicketSetting {
    pub per_user_ticket_limit: Option<i32>,
    pub allow_user_to_close_tickets: Option<bool>,
    pub ticket_close_confirmation: Option<bool>,
    pub ticket_notification_channel: Option<String>,
    pub transcripts_channel: Option<String>,
    pub thread_ticket: Option<bool>,
    pub archive_category: Option<String>,
    pub archive_overflow_category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketSetting {
    pub id: i32,
    pub per_user_ticket_limit: i32,
    pub allow_user_to_close_tickets: bool,
    pub ticket_close_confirmation: bool,
    pub ticket_notification_channel: Option<String>,
    pub transcripts_channel: Option<String>,
    pub bot_id: i32,
    pub guild_id: i32,
    pub thread_ticket: bool,
    pub archive_category: Option<String>,
    pub archive_overflow_category: Option<String>,
}

impl From<TicketSettingModel> for ResponseTicketSetting {
    fn from(model: TicketSettingModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            per_user_ticket_limit: model.per_user_ticket_limit,
            allow_user_to_close_tickets: model.allow_user_to_close_tickets != 0,
            ticket_close_confirmation: model.ticket_close_confirmation != 0,
            ticket_notification_channel: model.ticket_notification_channel,
            transcripts_channel: model.transcripts_channel,
            thread_ticket: model.thread_ticket != 0,
            archive_category: model.archive_category,
            archive_overflow_category: model.archive_overflow_category,
        }
    }
}

pub struct TicketSettingRoutes {}

impl UniqueBotGuildEntityRoutes for TicketSettingRoutes {}

#[async_trait]
impl DefaultRoutes for TicketSettingRoutes {
    type Queries = TicketSettingQueries;

    type ResponseJson = ResponseTicketSetting;

    fn path() -> String {
        format!("settings")
    }
}
