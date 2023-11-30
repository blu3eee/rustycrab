use serde::{ Serialize, Deserialize };

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
    pub bot_id: i32,
    pub guild_id: i32,
    pub per_user_ticket_limit: i32,
    pub allow_user_to_close_tickets: bool,
    pub ticket_close_confirmation: bool,
    pub ticket_notification_channel: Option<String>,
    pub transcripts_channel: Option<String>,
    pub thread_ticket: bool,
    pub archive_category: Option<String>,
    pub archive_overflow_category: Option<String>,
}
