use async_trait::async_trait;
use rustycrab_model::response::ticket::setting::ResponseTicketSetting;
use crate::{
    database::ticket_settings::Model as TicketSettingModel,
    default_router::DefaultRoutes,
    queries::tickets_system::ticket_setting_queries::TicketSettingQueries,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
};

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
