use rustycrab_api::{
    queries::{ marriages_queries::MarriageQueries, item_queries::BotItemQueries },
    default_queries::DefaultSeaQueries,
};
use rustycrab_model::{
    response::marriages::{ RequestCreateMarriage, RequestUpdateMarriage },
    error::BoxedError,
};
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait };
use tokio::task::JoinHandle;

use crate::old_database::{
    marriage as old_marriage,
    bot_user_info as old_botuser,
    items as old_items,
};

pub async fn migrate(old: DatabaseConnection, new: DatabaseConnection) -> Result<(), BoxedError> {
    let marriages = old_marriage::Entity::find().all(&old).await?;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for rela in marriages {
        let old_cloned = old.clone();
        let new_cloned = new.clone();

        let handle: JoinHandle<()> = tokio::spawn(async move {
            let bot_user1 = old_botuser::Entity
                ::find()
                .filter(old_botuser::Column::Id.eq(rela.user1_id.unwrap()))
                .one(&old_cloned).await
                .expect("error finding botuser1")
                .unwrap();
            let bot_user2 = old_botuser::Entity
                ::find()
                .filter(old_botuser::Column::Id.eq(rela.user2_id.unwrap()))
                .one(&old_cloned).await
                .expect("error finding botuser2")
                .unwrap();

            let bot_discord_id = bot_user1.bot_bot_id.unwrap();
            let user1_discord_id = bot_user1.user_discord_id.unwrap();

            let user2_discord_id = bot_user2.user_discord_id.unwrap();

            let mut marriage_create = RequestCreateMarriage {
                bot_discord_id: bot_discord_id.clone(),
                user1_discord_id,
                user2_discord_id,
                ..Default::default()
            };
            if let Some(ring_id) = rela.ring_id {
                if
                    let Some(old_ring) = old_items::Entity
                        ::find()
                        .filter(old_items::Column::Id.eq(ring_id))
                        .one(&old_cloned).await
                        .ok()
                {
                    if let Some(old_ring) = old_ring {
                        if
                            let Ok(new_ring) = BotItemQueries::find_by_item_id(
                                &new_cloned,
                                &bot_discord_id,
                                &old_ring.name
                            ).await
                        {
                            marriage_create.ring_id = Some(new_ring.id);
                        }
                    }
                }
            }

            let marriage_create = MarriageQueries::create_entity(
                &new_cloned,
                marriage_create
            ).await.expect("failed to create marriage");

            let update_result = MarriageQueries::update_by_id(
                &new_cloned,
                marriage_create.id,
                RequestUpdateMarriage {
                    image: rela.image_url,
                    thumbnail: rela.thumbnail_url,
                    caption: Some(rela.caption),
                    quote: Some(rela.quote),
                    ..Default::default()
                }
            ).await;

            match update_result {
                Ok(marriage) => {
                    println!("created marriage {}", marriage.id);
                }
                Err(e) => {
                    eprint!("failed to create marriage: {e:?}");
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
