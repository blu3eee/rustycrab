use rustycrab_model::response::ticket::ticket::{ RequestCreateTicket, RequestUpdateTicket };
use sea_orm::{
    DatabaseConnection,
    Set,
    EntityTrait,
    RelationTrait,
    QueryFilter,
    Condition,
    ColumnTrait,
};
use async_trait::async_trait;

use crate::{
    default_queries::DefaultSeaQueries,
    database::tickets::{ self, Entity as Tickets, ActiveModel as TicketActiveModel },
    utilities::app_error::AppError,
    queries::{
        tickets_system::ticket_panels_queries::TicketPanelsQueries,
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
    },
    multi_bot_guild_entities_queries::MultipleBotGuildEntityQueries,
};

pub struct TicketQueries {}

impl TicketQueries {
    pub async fn find_by_channel_discord_id(
        db: &DatabaseConnection,
        channel_id: String
    ) -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .filter(Condition::all().add(tickets::Column::ChannelId.eq(channel_id)))
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Record not found"))
    }

    pub async fn find_user_tickets(
        db: &DatabaseConnection,
        user_id: String
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .filter(Condition::all().add(tickets::Column::UserId.eq(user_id)))
            .all(db).await
            .map_err(AppError::from)
    }
}

#[async_trait]
impl DefaultSeaQueries for TicketQueries {
    type Entity = Tickets;
    type ActiveModel = TicketActiveModel;

    type CreateData = RequestCreateTicket;
    type UpdateData = RequestUpdateTicket;
    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let _ = TicketPanelsQueries::find_by_id(db, create_data.panel_id).await?;
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.guild_discord_id).await?;
        Self::save_active_model(db, Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            user_id: Set(create_data.user_id),
            panel_id: Set(create_data.panel_id),
            opened_time: Set(create_data.opened_time),
            ..Default::default()
        }).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.user_id {
            active_model.user_id = Set(value);
        }

        if let Some(value) = update_data.channel_id {
            active_model.channel_id = Set(Some(value));
        }

        if let Some(value) = update_data.status {
            active_model.status = Set(Some(value));
        }
        if let Some(value) = update_data.notification_message_id {
            active_model.notification_message_id = Set(Some(value));
        }
        if let Some(value) = update_data.transcript_message_id {
            active_model.transcript_message_id = Set(Some(value));
        }
        if let Some(value) = update_data.transcript_channel_id {
            active_model.transcript_channel_id = Set(Some(value));
        }

        Ok(())
    }
}

impl MultipleBotGuildEntityQueries for TicketQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        tickets::Relation::Bots.def()
    }

    fn guild_relation() -> sea_orm::entity::RelationDef {
        tickets::Relation::GuildInfo.def()
    }
}
