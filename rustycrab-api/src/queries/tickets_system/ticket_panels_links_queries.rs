use sea_orm::{
    DatabaseConnection,
    Set,
    DeleteResult,
    EntityTrait,
    QueryFilter,
    ColumnTrait,
    ActiveModelTrait,
};

use crate::{
    database::{
        ticket_multi_panels_panels_ticket_panels::{
            Entity as TicketPanelLinks,
            Model as TicketPanelLinkModel,
            ActiveModel as TicketPanelLinkActiveModel,
            Column,
        },
        ticket_panels::Model as TicketPanelModel,
    },
    utilities::app_error::AppError,
    queries::tickets_system::ticket_panels_queries::TicketPanelsQueries,
    default_queries::DefaultSeaQueries,
};

pub struct TicketPanelLinksQueries {}

impl TicketPanelLinksQueries {
    pub async fn create_entity(
        db: &DatabaseConnection,
        multi_panel_id: i32,
        panel_id: i32
    ) -> Result<TicketPanelLinkModel, AppError> {
        let active_model = TicketPanelLinkActiveModel {
            ticket_multi_panels_id: Set(multi_panel_id),
            ticket_panels_id: Set(panel_id),
        };
        active_model.insert(db).await.map_err(|error| {
            eprintln!("Error saving panel link: {:?}", error);
            AppError::internal_server_error("Error saving panel link")
        })
    }

    pub async fn delete_multi_panel_links(
        db: &DatabaseConnection,
        multi_panel_id: i32
    ) -> Result<DeleteResult, AppError> {
        TicketPanelLinks::delete_many()
            .filter(Column::TicketMultiPanelsId.eq(multi_panel_id))
            .exec(db).await
            .map_err(AppError::from)
    }

    pub async fn delete_panel_links(
        db: &DatabaseConnection,
        panel_id: i32
    ) -> Result<DeleteResult, AppError> {
        TicketPanelLinks::delete_many()
            .filter(Column::TicketPanelsId.eq(panel_id))
            .exec(db).await
            .map_err(AppError::from)
    }

    pub async fn delete(
        db: &DatabaseConnection,
        multi_panel_id: i32,
        panel_id: i32
    ) -> Result<DeleteResult, AppError> {
        TicketPanelLinks::delete_many()
            .filter(Column::TicketMultiPanelsId.eq(multi_panel_id))
            .filter(Column::TicketPanelsId.eq(panel_id))
            .exec(db).await
            .map_err(AppError::from)
    }

    pub async fn get_links(
        db: &DatabaseConnection,
        multi_panel_id: i32
    ) -> Result<Vec<TicketPanelLinkModel>, AppError> {
        TicketPanelLinks::find()
            .filter(Column::TicketMultiPanelsId.eq(multi_panel_id))
            .all(db).await
            .map_err(AppError::from)
    }

    pub async fn get_subpanels(
        db: &DatabaseConnection,
        multi_panel_id: i32
    ) -> Result<Vec<TicketPanelModel>, AppError> {
        let links = Self::get_links(db, multi_panel_id).await?;
        let mut models: Vec<TicketPanelModel> = Vec::new();
        for link in links {
            models.push(TicketPanelsQueries::find_by_id(db, link.ticket_panels_id).await?);
        }

        Ok(models)
    }
}
