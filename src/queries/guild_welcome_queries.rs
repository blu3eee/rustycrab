use async_trait::async_trait;
use rustycrab_model::response::bot_guild_welcome::{ RequestCreateWelcome, RequestUpdateWelcome };
use sea_orm::{
    DatabaseConnection,
    EntityTrait,
    Set,
    ActiveValue,
    RelationTrait,
    PrimaryKeyTrait,
    DeleteResult,
};
use crate::{
    database::bot_guild_welcomes::{
        self,
        Entity as GuildWelcomes,
        ActiveModel as GuildWelcomeActiveModel,
    },
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
    unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries,
};

use super::{
    bot_queries::BotQueries,
    guild_queries::GuildQueries,
    message_queries::MessageQueries,
};

pub struct GuildWelcomeQueries {}

impl GuildWelcomeQueries {}

impl UniqueBotGuildEntityQueries for GuildWelcomeQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        bot_guild_welcomes::Relation::Bots.def()
    }
    fn guild_relation() -> sea_orm::entity::RelationDef {
        bot_guild_welcomes::Relation::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for GuildWelcomeQueries {
    type Entity = GuildWelcomes;
    type ActiveModel = GuildWelcomeActiveModel;

    type CreateData = RequestCreateWelcome;
    type UpdateData = RequestUpdateWelcome;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if
            let Ok(welcome) = Self::find_by_discord_ids(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id
            ).await
        {
            return Ok(welcome);
        }
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.guild_discord_id).await?;

        let message = if let Some(message_data) = create_data.message_data {
            Some(MessageQueries::create_entity(db, message_data).await?)
        } else {
            None
        };

        let active_model: GuildWelcomeActiveModel = GuildWelcomeActiveModel {
            bot_id: Set(Some(bot.id)),
            guild_id: Set(Some(guild.id)),
            message_id: Set(message.map(|e| e.id)),
            channel_id: Set(create_data.channel_id),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        // Update channel_id if provided
        if let Some(channel_id) = update_data.channel_id {
            active_model.channel_id = Set(Some(channel_id));
        }

        // Handle message_data update
        if let Some(message_data) = update_data.message_data {
            if let ActiveValue::Unchanged(Some(message_id)) = active_model.message_id {
                let _ = MessageQueries::update_by_id(db, message_id, message_data).await?;
            } else {
                let message = MessageQueries::create_entity(db, message_data).await?;
                active_model.message_id = Set(Some(message.id));
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
        let model = Self::Entity::find_by_id(id.into())
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Guild welcome not found"))?;

        let result = Self::Entity::delete_by_id(model.id).exec(db).await.map_err(AppError::from);

        // Delete related message
        if let Some(message_id) = model.message_id {
            MessageQueries::delete_by_id(db, message_id).await?;
        }

        result
    }
}
