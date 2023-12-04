use std::{ collections::HashMap, sync::Arc };

/// this includes migrating old bots, guilds, and bot_guild_configs

use rustycrab_model::{
    error::BoxedError,
    response::{
        bots::RequestCreateBot,
        guilds::RequestCreateGuild,
        bot_guild_config::{ RequestCreateConfig, RequestUpdateConfig },
    },
};
use sea_orm::{ DatabaseConnection, EntityTrait };
use tokio::sync::RwLock;

use crate::old_database::{
    bots_info as old_bots,
    guild_info as old_guilds,
    bot_guild_configurations as old_configs,
};

use rustycrab_api::{
    database::{ bots as new_bots, guild_info as new_guilds },
    queries::{
        guild_config_queries::GuildConfigQueries,
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
    },
    default_queries::DefaultSeaQueries,
};

async fn migrate_bots(
    old: &DatabaseConnection,
    new: &DatabaseConnection
) -> Result<HashMap<String, new_bots::Model>, BoxedError> {
    let bots = old_bots::Entity::find().all(old).await?;

    let mut new_bots: HashMap<String, new_bots::Model> = HashMap::new();
    for bot in bots {
        let create_req = BotQueries::create_entity(new, RequestCreateBot {
            bot_id: bot.bot_id.clone(),
            token: bot.token,
            theme_hex_color: Some(bot.theme_hex_color),
            discord_callback_url: Some(bot.discord_callback_url),
            discord_secret: Some(bot.discord_secret),
        }).await;

        match create_req {
            Ok(created) => {
                new_bots.entry(created.bot_id.clone()).or_insert(created.clone());
                println!("created bot {}", created.bot_id);
            }
            Err(e) => {
                eprintln!("error creating bot {}: {:?}", bot.bot_id, e);
            }
        }
    }

    Ok(new_bots)
}

async fn migrate_guilds(
    old: &DatabaseConnection,
    new: &DatabaseConnection
) -> Result<HashMap<String, new_guilds::Model>, BoxedError> {
    let guilds = old_guilds::Entity::find().all(old).await?;

    let mut new_guilds: HashMap<String, new_guilds::Model> = HashMap::new();
    for guild in guilds {
        let create_req = GuildQueries::create_entity(new, RequestCreateGuild {
            guild_id: guild.guild_id.clone(),
        }).await;

        match create_req {
            Ok(created) => {
                new_guilds.entry(created.guild_id.clone()).or_insert(created.clone());
                println!("created guild  {}", created.guild_id);
            }
            Err(e) => {
                eprintln!("error creating guild {}: {:?}", guild.guild_id, e);
            }
        }
    }

    Ok(new_guilds)
}

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let new_bots = Arc::new(RwLock::new(migrate_bots(&old, &new).await?));
    let new_guilds = Arc::new(RwLock::new(migrate_guilds(&old, &new).await?));

    let configs = old_configs::Entity::find().all(&old).await?;
    println!("found {} configs ", configs.len());
    for config in configs.clone() {
        // let new_bots_clone = Arc::clone(&new_bots);
        // let new_guilds_clone = Arc::clone(&new_guilds);
        if let Some(bot_id) = config.bot_bot_id {
            if let Some(guild_id) = config.guild_guild_id {
                let new_bots_read = new_bots.read().await;
                let new_guilds_read = new_guilds.read().await;

                let bot = new_bots_read.get(&bot_id).cloned().unwrap();
                let guild = new_guilds_read.get(&guild_id).cloned().unwrap();
                let created_config = GuildConfigQueries::create_entity(&new, RequestCreateConfig {
                    bot_discord_id: bot.bot_id.clone(),
                    guild_discord_id: guild.guild_id.clone(),
                }).await.expect("failed to create config");

                let update_result = GuildConfigQueries::update_by_id(
                    &new,
                    created_config.id,
                    RequestUpdateConfig {
                        bot_id: bot.id,
                        guild_id: guild.id,
                        prefix: Some(config.prefix),
                        locale: Some(config.locale),
                        ..Default::default()
                    }
                ).await;

                match update_result {
                    Ok(new_config) => {
                        println!("created config {} {}", new_config.bot_id, new_config.guild_id);
                    }
                    Err(e) => {
                        eprintln!("failed to update guild config: {:?}", e);
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn find_and_create_config(
    new: DatabaseConnection,
    new_bots: Arc<RwLock<HashMap<String, new_bots::Model>>>,
    new_guilds: Arc<RwLock<HashMap<String, new_guilds::Model>>>,
    config: old_configs::Model
) -> Result<(), BoxedError> {
    println!("config find_and_create_config");
    if let Some(bot_id) = config.bot_bot_id {
        if let Some(guild_id) = config.guild_guild_id {
            let new_bots_read = new_bots.read().await;
            let new_guilds_read = new_guilds.read().await;

            let bot = new_bots_read.get(&bot_id).cloned().unwrap();
            let guild = new_guilds_read.get(&guild_id).cloned().unwrap();
            let created_config = GuildConfigQueries::create_entity(&new, RequestCreateConfig {
                bot_discord_id: bot.bot_id.clone(),
                guild_discord_id: guild.guild_id.clone(),
            }).await.expect("failed to create config");

            let update_result = GuildConfigQueries::update_by_id(
                &new,
                created_config.id,
                RequestUpdateConfig {
                    bot_id: bot.id,
                    guild_id: guild.id,
                    prefix: Some(config.prefix),
                    locale: Some(config.locale),
                    ..Default::default()
                }
            ).await;

            match update_result {
                Ok(new_config) => {
                    println!("created config {} {}", new_config.bot_id, new_config.guild_id);
                }
                Err(e) => {
                    eprintln!("failed to update guild config: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
