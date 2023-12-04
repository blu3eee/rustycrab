use axum::Router;

use crate::{
    default_router::DefaultRoutes,
    unique_bot_guild_entity_router::UniqueBotGuildEntityRoutes,
    multi_bot_guild_entities_router::MultipleBotGuildEntitiesRoutes,
};

use self::{
    ticket_settings::TicketSettingRoutes,
    ticket_support_teams::TicketSupportTeamRoutes,
    ticket_panels::TicketPanelsRoutes,
    ticket_multipanels::TicketMultiPanelsRoutes,
    tickets::TicketsRoutes,
};

pub mod ticket_settings;
pub mod ticket_multipanels;
pub mod ticket_panels;
pub mod ticket_support_teams;
pub mod tickets;

pub async fn ticket_routes() -> Router {
    Router::new().nest(
        "/ticket",
        Router::new()
            .merge(<TicketSettingRoutes as UniqueBotGuildEntityRoutes>::router().await)
            .merge(TicketSupportTeamRoutes::router().await)
            .merge(TicketPanelsRoutes::router().await)
            .merge(TicketMultiPanelsRoutes::router().await)
            .merge(<TicketsRoutes as MultipleBotGuildEntitiesRoutes>::router().await)
    )
}
