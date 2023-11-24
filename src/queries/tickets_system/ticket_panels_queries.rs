use sea_orm::{
    DatabaseConnection,
    Set,
    RelationTrait,
    EntityTrait,
    JoinType::LeftJoin,
    Condition,
    ColumnTrait,
    QuerySelect,
    QueryFilter,
    ActiveValue,
    DeleteResult,
    PrimaryKeyTrait,
};
use async_trait::async_trait;

use crate::{
    default_queries::DefaultSeaQueries,
    database::{
        ticket_panels::{ self, Entity as TicketPanels, ActiveModel as TicketPanelActiveModel },
        bots,
        guild_info,
    },
    router::routes::tickets::ticket_panels::{ RequestCreateTicketPanel, RequestUpdateTicketPanel },
    queries::{
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
        message_queries::MessageQueries,
        message_button_queries::MessageButtonQueries,
    },
    utilities::app_error::AppError,
};

use super::ticket_panels_links_queries::TicketPanelLinksQueries;

pub struct TicketPanelsQueries {}

impl TicketPanelsQueries {
    pub async fn find_panels_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(LeftJoin, ticket_panels::Relation::Bots.def())
            .join(LeftJoin, ticket_panels::Relation::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .all(db).await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl DefaultSeaQueries for TicketPanelsQueries {
    type Entity = TicketPanels;
    type ActiveModel = TicketPanelActiveModel;

    type CreateData = RequestCreateTicketPanel;
    type UpdateData = RequestUpdateTicketPanel;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.bot_discord_id).await?;

        let message = MessageQueries::create_entity(db, create_data.message_data).await?;
        let button = MessageButtonQueries::create_entity(db, create_data.button_data).await?;
        let welcome_message = MessageQueries::create_entity(
            db,
            create_data.welcome_message_data
        ).await?;

        Self::save_active_model(db, Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            mention_on_open: Set(create_data.mention_on_open.join(",")),
            naming_scheme: Set(create_data.naming_scheme),
            channel_id: Set(create_data.channel_id),
            message_id: Set(Some(message.id)),
            button_id: Set(Some(button.id)),
            welcome_message_id: Set(Some(welcome_message.id)),
            support_team_id: Set(Some(create_data.support_team_id)),
            ..Default::default()
        }).await
    }

    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.mention_on_open {
            active_model.mention_on_open = Set(value.join(","));
        }

        if let Some(value) = update_data.naming_scheme {
            active_model.naming_scheme = Set(value);
        }

        if let Some(value) = update_data.sent_message_id {
            active_model.sent_message_id = Set(value);
        }

        if let Some(value) = update_data.channel_id {
            active_model.channel_id = Set(value);
        }

        if let Some(value) = update_data.support_team_id {
            active_model.support_team_id = Set(Some(value));
        }

        println!("Current message_id in active_model: {:?}", active_model.message_id);
        if let Some(data) = update_data.message_data {
            println!("Message data exists: {:?}", data);
            if let ActiveValue::Unchanged(Some(id)) = active_model.message_id {
                println!("Updating message with ID: {}", id);
                MessageQueries::update_by_id(db, id, data).await?;
            } else {
                println!("No message_id in active_model to update");
            }
        }

        if let Some(data) = update_data.welcome_message_data {
            if let ActiveValue::Unchanged(Some(id)) = active_model.welcome_message_id {
                MessageQueries::update_by_id(db, id, data).await?;
            }
        }

        if let Some(data) = update_data.button_data {
            if let ActiveValue::Unchanged(Some(id)) = active_model.button_id {
                MessageButtonQueries::update_by_id(db, id, data).await?;
            }
        }

        Ok(())
    }

    async fn delete_by_id<K>(db: &DatabaseConnection, id: K) -> Result<DeleteResult, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        // Retrieve the ticket panel
        let ticket_panel = Self::Entity::find_by_id(id.into())
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Ticket panel not found"))?;

        // Delete related message and button entities
        if let Some(message_id) = ticket_panel.message_id {
            MessageQueries::delete_by_id(db, message_id).await?;
        }
        if let Some(welcome_message_id) = ticket_panel.welcome_message_id {
            MessageQueries::delete_by_id(db, welcome_message_id).await?;
        }
        if let Some(button_id) = ticket_panel.button_id {
            MessageButtonQueries::delete_by_id(db, button_id).await?;
        }

        TicketPanelLinksQueries::delete_panel_links(db, ticket_panel.id).await?;

        // Finally, delete the ticket panel
        Self::Entity::delete_by_id(ticket_panel.id).exec(db).await.map_err(AppError::from)
    }
}
