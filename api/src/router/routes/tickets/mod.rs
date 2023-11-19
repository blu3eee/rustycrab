use axum::Router;

use crate::{
    app_state::AppState,
    default_router::DefaultRoutes,
    bot_guild_entity_router::BotGuildEntityRoutes,
};

use self::{
    ticket_settings::TicketSettingRoutes,
    ticket_support_teams::TicketSupportTeamRoutes,
    ticket_panels::TicketPanelsRoutes,
    ticket_multipanels::TicketMultiPanelsRoutes,
};

pub mod ticket_settings;
pub mod ticket_multipanels;
pub mod ticket_panels;
pub mod ticket_support_teams;

pub async fn ticket_routes(state: AppState) -> Router {
    Router::new().nest(
        "/tickets",
        Router::new()
            .merge(<TicketSettingRoutes as BotGuildEntityRoutes>::router(state.clone()).await)
            .merge(TicketSupportTeamRoutes::router(state.clone()).await)
            .merge(TicketPanelsRoutes::router(state.clone()).await)
            .merge(TicketMultiPanelsRoutes::router(state.clone()).await)
    )
}
