use sea_orm::{ DatabaseConnection, Set, RelationTrait, EntityTrait };
use async_trait::async_trait;

use crate::{
    default_queries::DefaultSeaQueries,
    database::ticket_settings::{
        Entity as TicketSettings,
        ActiveModel as TicketSettingActiveModel,
        Relation as TicketSettingRelations,
    },
    router::routes::tickets::ticket_settings::{
        RequestCreateTicketSetting,
        RequestUpdateTicketSetting,
    },
    bot_guild_entity_queries::BotGuildEntityQueries,
    utilities::app_error::AppError,
    queries::{ bot_queries::BotQueries, guild_queries::GuildQueries },
};

pub struct TicketSettingQueries {}

impl TicketSettingQueries {}

impl BotGuildEntityQueries for TicketSettingQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        TicketSettingRelations::Bots.def()
    }
    fn guild_relation() -> sea_orm::entity::RelationDef {
        TicketSettingRelations::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for TicketSettingQueries {
    type Entity = TicketSettings;

    type ActiveModel = TicketSettingActiveModel;

    type CreateData = RequestCreateTicketSetting;
    type UpdateData = RequestUpdateTicketSetting;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if
            let Ok(model) = Self::find_by_discord_ids(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id
            ).await
        {
            return Ok(model);
        }

        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.bot_discord_id).await?;

        let active_model = Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),

            archive_category: Set(create_data.archive_category.map(|value| value)),
            archive_overflow_category: Set(
                create_data.archive_overflow_category.map(|value| value)
            ),
            transcripts_channel: Set(create_data.transcripts_channel.map(|value| value)),
            ticket_notification_channel: Set(
                create_data.ticket_notification_channel.map(|value| value)
            ),
            allow_user_to_close_tickets: if
                let Some(value) = create_data.allow_user_to_close_tickets
            {
                Set(value as i8)
            } else {
                Set(false as i8)
            },
            thread_ticket: if let Some(value) = create_data.thread_ticket {
                Set(value as i8)
            } else {
                Set(false as i8)
            },
            per_user_ticket_limit: if let Some(value) = create_data.per_user_ticket_limit {
                Set(value)
            } else {
                Set(5)
            },
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        // Apply updates from the DTO

        if let Some(value) = update_data.archive_category {
            active_model.archive_category = Set(Some(value));
        }

        if let Some(value) = update_data.archive_overflow_category {
            active_model.archive_overflow_category = Set(Some(value));
        }

        if let Some(value) = update_data.transcripts_channel {
            active_model.transcripts_channel = Set(Some(value));
        }

        if let Some(value) = update_data.ticket_notification_channel {
            active_model.ticket_notification_channel = Set(Some(value));
        }

        if let Some(value) = update_data.allow_user_to_close_tickets {
            active_model.allow_user_to_close_tickets = Set(value as i8);
        } else {
            active_model.allow_user_to_close_tickets = Set(false as i8);
        }

        if let Some(value) = update_data.thread_ticket {
            active_model.thread_ticket = Set(value as i8);
        } else {
            active_model.thread_ticket = Set(true as i8);
        }

        if let Some(value) = update_data.ticket_close_confirmation {
            active_model.ticket_close_confirmation = Set(value as i8);
        } else {
            active_model.ticket_close_confirmation = Set(true as i8);
        }

        if let Some(value) = update_data.per_user_ticket_limit {
            active_model.per_user_ticket_limit = Set(value);
        } else {
            active_model.per_user_ticket_limit = Set(5);
        }

        Ok(())
    }
}
