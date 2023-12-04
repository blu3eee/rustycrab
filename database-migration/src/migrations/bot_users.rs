use crate::old_database::bot_user_info as old_botusers;

use rustycrab_api::{
    queries::{ bot_user_queries::BotUserQueries, user_queries::UserQueries },
    default_queries::DefaultSeaQueries,
};
use rustycrab_model::{
    response::bot_users::{ RequestCreateBotUser, RequestUpdateBotUser },
    error::BoxedError,
};
use sea_orm::{ DatabaseConnection, EntityTrait };
use tokio::task::JoinHandle;
use std::collections::HashSet;

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let bot_users = old_botusers::Entity
        ::find()
        .all(&old).await
        .expect("failed to find all old bot users");

    let mut handles: Vec<JoinHandle<Result<(), BoxedError>>> = Vec::new();

    let user_ids_vec: Vec<String> = bot_users
        .clone()
        .into_iter()
        .filter_map(|bot_user| bot_user.user_discord_id)
        .collect();

    let user_ids_set: HashSet<String> = user_ids_vec.into_iter().collect();

    println!("[migrating] check and creating {} users", user_ids_set.len());

    // Spawn tasks for creating users and collect their handles
    let mut user_creation_handles: Vec<JoinHandle<()>> = Vec::new();
    for user_id in user_ids_set {
        let new_clone = new.clone();
        let handle = tokio::spawn(async move {
            let _ = UserQueries::find_user_or_create(&new_clone, &user_id).await;
        });
        user_creation_handles.push(handle);
    }

    // Await all user creation tasks to complete
    for handle in user_creation_handles {
        handle.await?;
    }

    println!("finished finding and creating users");

    for bot_user in bot_users {
        let new_clone = new.clone();

        // Spawn a task for each bot_user migration
        let handle: JoinHandle<Result<(), BoxedError>> = tokio::spawn(async move {
            // Your migration logic here...
            // Replace `?` with proper error handling if necessary
            println!("[migrating] bot_user {}", bot_user.id);

            let bot_discord_id = bot_user.bot_bot_id
                .ok_or("unable to unwrap bot discord id ")
                .expect("");

            let user_discord_id = bot_user.user_discord_id
                .ok_or("unable to unwrap uslser discord id")
                .expect("");

            let new_bot_user = BotUserQueries::create_entity(&new_clone, RequestCreateBotUser {
                bot_discord_id: bot_discord_id.clone(),
                user_discord_id: user_discord_id.clone(),
            }).await.expect("unable to create bot user");

            let update_result = BotUserQueries::update_by_id(
                &new_clone,
                new_bot_user.id,
                RequestUpdateBotUser {
                    balance: Some(bot_user.balance),
                    pray_points: Some(bot_user.pray_points),
                    inventory: if let Some(inv) = bot_user.inventory {
                        Some(inv)
                    } else {
                        Some(String::new())
                    },
                }
            ).await;

            match update_result {
                Ok(_) => {
                    println!("created bot_user {} {}", bot_discord_id, user_discord_id);
                }
                Err(e) => {
                    eprintln!(
                        "failed to update bot user {} {}: {:?}",
                        bot_discord_id,
                        user_discord_id,
                        e
                    );
                }
            }

            // Return Result<(), BoxedError> from the task
            Ok(())
        });

        // Store the handle if you need to wait for the task's result
        handles.push(handle);
    }

    // Await all handles to ensure all tasks are completed
    for handle in handles {
        handle.await??;
    }

    Ok(())
}
