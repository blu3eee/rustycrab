use async_trait::async_trait;
use rustycrab_model::response::ticket::ticket::ResponseTicket;

use crate::{
    database::tickets::Model as TicketModel,
    default_router::DefaultRoutes,
    queries::tickets_system::ticket_queries::TicketQueries,
    multi_bot_guild_entities_router::MultipleBotGuildEntitiesRoutes,
};

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
