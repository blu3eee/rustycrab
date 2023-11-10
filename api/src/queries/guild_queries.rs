use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set };
use crate::database::guild_info::{
    Entity as Guild,
    Model as GuildModel,
    ActiveModel as GuildActiveModel,
    self,
};
use crate::utilities::app_error::AppError;

pub async fn get_one_guild(
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

pub async fn get_one_guild_or_create(
    db: &DatabaseConnection,
    discord_id: &str
) -> Result<GuildModel, AppError> {
    match Guild::find().filter(guild_info::Column::GuildId.eq(discord_id)).one(db).await {
        Ok(Some(guild)) => {
            // If the guild is found, return it
            Ok(guild)
        }
        Ok(None) => {
            // If the guild is not found, create a new one
            let new_guild = GuildActiveModel {
                guild_id: Set(discord_id.to_owned()),
                // Set other fields as needed, for example:
                // ... other fields ...
                ..Default::default() // Use default values for the rest of the fields
            };

            new_guild.insert(db).await.map_err(|err| {
                eprintln!("Error creating new guild: {:?}", err);
                AppError::internal_server_error("There was an error creating the guild")
            })
        }
        Err(err) => {
            // If there's an error querying the database, return an error
            eprintln!("Error getting guild from discordId: {:?}", err);
            Err(AppError::internal_server_error("There was an error getting the guild"))
        }
    }
}
