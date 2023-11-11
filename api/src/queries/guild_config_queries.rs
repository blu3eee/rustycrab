// queries/guild_config_queries.rs
use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QuerySelect,
    RelationTrait,
    Condition,
    JoinType::LeftJoin,
    ActiveModelTrait,
    Set,
};

use crate::{
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
    database::{
        bot_guild_configurations::{
            self,
            Entity as GuildConfig,
            Model as GuildConfigModel,
            Relation as GuildConfigRelations,
        },
        bots,
        guild_info,
    },
    routes::guild_configs::{ RequestCreateConfig, RequestUpdateConfig },
};

use super::{
    bot_queries::get_bot_from_discord_id,
    guild_queries::get_one_guild_or_create,
    save_active_model,
};

pub async fn get_all_bot_configs(
    db: &DatabaseConnection,
    bot_id: &str
) -> Result<Vec<GuildConfigModel>, AppError> {
    // Find the configuration for the given bot_id and guild_id.
    GuildConfig::find()
        .join(LeftJoin, GuildConfigRelations::Bots.def())
        .filter(bots::Column::BotId.eq(bot_id))
        .all(db).await
        .map_err(convert_seaorm_error)
}

pub async fn get_one_config(
    db: &DatabaseConnection,
    bot_id: &str,
    guild_id: &str
) -> Result<GuildConfigModel, AppError> {
    log::debug!("Query looking for guild config with bot_id {} and guild_id {}", bot_id, guild_id);

    // Find the configuration for the given bot_id and guild_id.
    let config = GuildConfig::find()
        .join(LeftJoin, GuildConfigRelations::Bots.def())
        .join(LeftJoin, GuildConfigRelations::GuildInfo.def())
        .filter(
            Condition::all()
                .add(bots::Column::BotId.eq(bot_id))
                .add(guild_info::Column::GuildId.eq(guild_id))
        )
        .one(db).await
        .map_err(convert_seaorm_error)?; // Convert the error to your application's error type
    // println!("{:?}", config);
    // Check if a configuration is found, otherwise return an error.
    match config {
        Some(cfg) => Ok(cfg),
        None => {
            log::warn!("Configuration not found for bot_id {} and guild_id {}", bot_id, guild_id);
            Err(AppError::not_found("Configuration not found".to_string()))
        }
    }
}

pub async fn create_config(
    db: &DatabaseConnection,
    create_dto: RequestCreateConfig
) -> Result<GuildConfigModel, AppError> {
    if
        let Ok(Some(config)) = GuildConfig::find()
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(&create_dto.guild_discord_id))
                    .add(guild_info::Column::GuildId.eq(&create_dto.bot_discord_id))
            )

            .one(db).await
    {
        return Ok(config);
    }

    // At this point, either the config wasn't found, or there was an error fetching it.
    // Either way, we proceed to create a new config.

    let guild = get_one_guild_or_create(db, &create_dto.guild_discord_id).await?;
    let bot = get_bot_from_discord_id(db, &create_dto.bot_discord_id).await?;

    let new_config: bot_guild_configurations::ActiveModel = bot_guild_configurations::ActiveModel {
        bot_id: Set(Some(bot.id)),
        guild_id: Set(Some(guild.id)),
        ..Default::default()
    };

    save_active_model(db, new_config).await
}

pub async fn update_config(
    db: &DatabaseConnection,
    bot_id: &i32,
    guild_id: &i32,
    update_dto: RequestUpdateConfig
) -> Result<GuildConfigModel, AppError> {
    // Find the existing guild configuration
    let mut config: bot_guild_configurations::ActiveModel = GuildConfig::find()
        .filter(
            Condition::all()
                .add(bot_guild_configurations::Column::BotId.eq(*bot_id))
                .add(bot_guild_configurations::Column::GuildId.eq(*guild_id))
        )
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Guild configuration not found"))?
        .into(); // Convert to ActiveModel for update

    // Update the fields if they have Some value
    if let Some(prefix) = update_dto.prefix {
        config.prefix = Set(prefix);
    }
    if let Some(locale) = update_dto.locale {
        config.locale = Set(locale);
    }
    if let Some(module_flags) = update_dto.module_flags {
        config.module_flags = Set(module_flags);
    }
    if let Some(premium_flags) = update_dto.premium_flags {
        config.premium_flags = Set(premium_flags);
    }

    // Save the updated configuration back to the database
    config.update(db).await.map_err(convert_seaorm_error)
}
