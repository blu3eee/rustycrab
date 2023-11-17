use async_trait::async_trait;
use sea_orm::{ DatabaseConnection, EntityTrait, Set, ActiveValue, RelationTrait };
use crate::{
    database::{
        bot_guild_welcomes::{
            self,
            Entity as GuildWelcomes,
            ActiveModel as GuildWelcomeActiveModel,
        },
        bots,
        guild_info,
    },
    routes::guild_welcomes::{ RequestCreateWelcome, RequestUpdateWelcome },
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
    bot_guild_entity_queries::BotGuildEntityQueries,
};

use super::{
    save_active_model,
    bot_queries::BotQueries,
    guild_queries::GuildQueries,
    message_queries::MessageQueries,
};

pub struct GuildWelcomeQueries {}

impl GuildWelcomeQueries {}

impl BotGuildEntityQueries for GuildWelcomeQueries {
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
        let guild: guild_info::Model = GuildQueries::find_one_or_create(
            db,
            &create_data.guild_discord_id
        ).await?;
        let bot: bots::Model = BotQueries::find_by_discord_id(
            db,
            &create_data.bot_discord_id
        ).await?;
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

        save_active_model(db, active_model).await
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
            if let ActiveValue::Set(Some(message_id)) = active_model.message_id {
                let _ = MessageQueries::update_by_id(db, message_id, message_data).await?;
            } else {
                let message = MessageQueries::create_entity(db, message_data).await?;
                active_model.message_id = Set(Some(message.id));
            }
        }

        Ok(())
    }
}

// pub async fn create_welcome(
//     db: &DatabaseConnection,
//     create_data: RequestCreateWelcome
// ) -> Result<GuildWelcomeModel, AppError> {
//     if
//         let Ok(Some(welcome)) = GuildWelcome::find()
//             .filter(
//                 Condition::all()
//                     .add(bots::Column::BotId.eq(&create_data.guild_discord_id))
//                     .add(guild_info::Column::GuildId.eq(&create_data.bot_discord_id))
//             )

//             .one(db).await
//     {
//         return Ok(welcome);
//     }
//     let guild: guild_info::Model = GuildQueries::find_one_or_create(
//         db,
//         &create_data.guild_discord_id
//     ).await?;
//     let bot: bots::Model = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
//     let message: Option<crate::routes::ResponseMessage> = if
//         let Some(message_data) = create_data.message_data
//     {
//         Some(create_message(db, message_data).await?)
//     } else {
//         None
//     };

//     let active_model: GuildWelcomeActiveModel = GuildWelcomeActiveModel {
//         bot_id: Set(Some(bot.id)),
//         guild_id: Set(Some(guild.id)),
//         message_id: Set(message.map(|e| e.id)),
//         channel_id: Set(create_data.channel_id),
//         ..Default::default()
//     };

//     save_active_model(db, active_model).await
// }

// pub async fn get_welcome(db: &DatabaseConnection, id: &i32) -> Result<GuildWelcomeModel, AppError> {
//     GuildWelcome::find_by_id(*id)
//         .one(db).await
//         .map_err(convert_seaorm_error)?
//         .ok_or_else(|| AppError::not_found("Welcome message not found"))
// }

// pub async fn get_welcome_from_discord_id(
//     db: &DatabaseConnection,
//     bot_id: &str,
//     guild_id: &str
// ) -> Result<GuildWelcomeModel, AppError> {
//     GuildWelcome::find()
//         .filter(
//             Condition::all()
//                 .add(bots::Column::BotId.eq(bot_id))
//                 .add(guild_info::Column::GuildId.eq(guild_id))
//         )
//         .one(db).await
//         .map_err(convert_seaorm_error)?
//         .ok_or_else(|| AppError::not_found("Welcome message not found"))
// }

// pub async fn get_welcome_response(
//     db: &DatabaseConnection,
//     id: &i32
// ) -> Result<ResponseGuildWelcomeDetails, AppError> {
//     let welcome: GuildWelcomeModel = get_welcome(db, id).await?;

//     let guild: crate::routes::guilds::ResponseGuild = GuildQueries::find_by_id(
//         db,
//         welcome.guild_id.unwrap()
//     ).await?.into();

//     let bot = BotQueries::find_by_id(db, welcome.bot_id.unwrap()).await?.into();

//     let message = fetch_message_response(db, &welcome.message_id).await?;

//     Ok(ResponseGuildWelcomeDetails {
//         id: welcome.id,
//         channel_id: welcome.channel_id,
//         enabled: welcome.enabled,
//         bot: Some(bot),
//         guild: Some(guild),
//         message,
//     })
// }

// pub async fn update_welcome(
//     db: &DatabaseConnection,
//     id: i32,
//     update_data: RequestUpdateWelcome
// ) -> Result<GuildWelcomeModel, AppError> {
//     let mut welcome: GuildWelcomeActiveModel = GuildWelcome::find_by_id(id)
//         .one(db).await
//         .map_err(convert_seaorm_error)?
//         .ok_or_else(|| AppError::not_found("Welcome message not found"))?
//         .into(); // Convert to ActiveModel for update

//     // Update channel_id if provided
//     if let Some(channel_id) = update_data.channel_id {
//         welcome.channel_id = Set(Some(channel_id));
//     }

//     // Handle message_data update
//     if let Some(message_data) = update_data.message_data {
//         if let ActiveValue::Set(Some(message_id)) = welcome.message_id {
//             let _ = update_message(db, &message_id, message_data).await?;
//         } else {
//             let message = create_message(db, message_data).await?;
//             welcome.message_id = Set(Some(message.id));
//         }
//     }

//     // Save the updated configuration back to the database
//     save_active_model(db, welcome).await
// }
