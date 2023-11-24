use async_trait::async_trait;
use serde::{ Serialize, Deserialize };
use crate::{
    database::tickets::Model as TicketModel,
    default_router::DefaultRoutes,
    queries::tickets_system::ticket_queries::TicketQueries,
    multi_bot_guild_entities_router::MultipleBotGuildEntitiesRoutes,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicket {
    pub id: i32,
    pub bot_id: i32,
    pub guild_id: i32,
    pub panel_id: i32,
    pub channel_id: Option<String>,
    pub opened_time: i32,
    pub user_id: String,
    pub status: Option<String>,
    pub notification_message_id: Option<String>,
    pub transcript_message_id: Option<String>,
    pub transcript_channel_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateTicket {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub panel_id: i32,
    pub user_id: String,
    pub opened_time: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestUpdateTicket {
    pub user_id: Option<String>,
    pub channel_id: Option<String>,
    pub status: Option<String>,
    pub notification_message_id: Option<String>,
    pub transcript_message_id: Option<String>,
    pub transcript_channel_id: Option<String>,
}

impl From<TicketModel> for ResponseTicket {
    fn from(model: TicketModel) -> Self {
        ResponseTicket {
            id: model.id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            panel_id: model.panel_id,
            channel_id: model.channel_id,
            opened_time: model.opened_time,
            user_id: model.user_id,
            status: model.status,
            notification_message_id: model.notification_message_id,
            transcript_message_id: model.transcript_message_id,
            transcript_channel_id: model.transcript_channel_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketTranscript {
    pub generated: i32,
    pub guild: TranscriptGuild,
    pub channel: TranscriptChannel,
    pub ticket_opener_id: String,
    pub users: Vec<TranscriptUser>,
    pub messages: Vec<TranscriptMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptGuild {
    pub guild_id: String,
    pub name: String,
    pub icon_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptUser {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
    pub bot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptChannel {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptMessage {
    pub user_id: String,
    pub content: String,
    pub embeds: Vec<TranscriptMessageEmbed>,
    pub attachments: Vec<TranscriptAttachment>,
    pub timestamp: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptMessageEmbed {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub timstamp: i32,
    pub color: i32,
    pub author: Option<TranscriptMessageEmbedAuthor>,
    pub footer: Option<TranscriptMessageEmbedFooter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptMessageEmbedAuthor {
    pub name: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptMessageEmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptAttachment {
    pub name: String,
    pub url: String,
}

pub struct TicketsRoutes {}

#[async_trait]
impl DefaultRoutes for TicketsRoutes {
    type Queries = TicketQueries;

    type ResponseJson = ResponseTicket;

    fn path() -> String {
        format!("tickets")
    }
}

impl MultipleBotGuildEntitiesRoutes for TicketsRoutes {}
