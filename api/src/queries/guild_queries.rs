use async_trait::async_trait;
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set };
use crate::database::guild_info::{
    Entity as Guild,
    Model as GuildModel,
    ActiveModel as GuildActiveModel,
    self,
};
use crate::default_queries::DefaultSeaQueries;
use crate::routes::guilds::{ RequestCreateGuild, RequestUpdateGuild };
use crate::utilities::app_error::AppError;

pub struct GuildQueries {}

impl GuildQueries {
    pub async fn find_by_discord_id(
        db: &DatabaseConnection,
        discord_id: &str
    ) -> Result<GuildModel, AppError> {
        Guild::find()
            .filter(guild_info::Column::GuildId.eq(discord_id))
            .one(db).await
            .map_err(|err| {
                eprintln!("Error getting bot from discordId: {:?}", err);
                AppError::internal_server_error("There was an error getting the bot")
            })
            .and_then(|bot| bot.ok_or_else(|| AppError::not_found("Guild not found")))
    }

    pub async fn find_one_or_create(
        db: &DatabaseConnection,
        discord_id: &str
    ) -> Result<GuildModel, AppError> {
        match Self::find_by_discord_id(db, discord_id).await {
            Ok(guild) => {
                // If the guild is found, return it
                Ok(guild)
            }
            Err(_) => {
                // If the guild is not found, create a new one
                let active_model = GuildActiveModel {
                    guild_id: Set(discord_id.to_owned()),
                    // Set other fields as needed, for example:
                    // ... other fields ...
                    ..Default::default() // Use default values for the rest of the fields
                };

                Self::save_active_model(db, active_model).await
            }
        }
    }
}

#[async_trait]
impl DefaultSeaQueries for GuildQueries {
    type Entity = guild_info::Entity;
    type ActiveModel = guild_info::ActiveModel;

    type CreateDto = RequestCreateGuild;
    type UpdateDto = RequestUpdateGuild;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateDto
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        if let Ok(model) = Self::find_by_discord_id(db, &create_data.guild_id).await {
            return Ok(model);
        }

        let active_model = guild_info::ActiveModel {
            guild_id: Set(create_data.guild_id),
            ..Default::default() // Set other fields to default if necessary
        };

        Self::save_active_model(db, active_model).await
    }

    #[allow(unused_variables)]
    fn apply_updates(
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateDto
    ) -> Result<(), AppError> {
        // Apply updates from the DTO

        Ok(())
    }
}
