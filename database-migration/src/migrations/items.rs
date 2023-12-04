use rustycrab_api::{ queries::item_queries::BotItemQueries, default_queries::DefaultSeaQueries };
use rustycrab_model::{
    error::BoxedError,
    response::items::{ RequestCreateBotItem, RequestUpdateBotItem },
};
use sea_orm::{ DatabaseConnection, EntityTrait };
use tokio::task::JoinHandle;

use crate::old_database::items as old_items;

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let old_items_list = old_items::Entity::find().all(&old).await?;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for old_item in old_items_list {
        let new_cloned = new.clone();

        let handle = tokio::spawn(async move {
            let new_item = BotItemQueries::create_entity(&new_cloned, RequestCreateBotItem {
                bot_discord_id: old_item.bot_bot_id.clone().unwrap(),
                item_id: old_item.item_id.to_string(),
                name: old_item.name.clone(),
            }).await.expect("failed to create item");

            let updated_result = BotItemQueries::update_by_id(
                &new_cloned,
                new_item.id,
                RequestUpdateBotItem {
                    emoji: old_item.emoji,
                    value: old_item.value,
                    functions: Some(
                        old_item.functions
                            .split(",")
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>()
                    ),
                    ..Default::default()
                }
            ).await;
            match updated_result {
                Ok(item) => { println!("[{:?}] created item {}", old_item.bot_bot_id, item.name) }
                Err(e) => {
                    println!(
                        "[{:?}] failed to create item {} - Err: {e:?}",
                        old_item.bot_bot_id,
                        old_item.name
                    )
                }
            }
        });
        handles.push(handle);
    }

    // Await all handles to ensure all tasks are completed
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
