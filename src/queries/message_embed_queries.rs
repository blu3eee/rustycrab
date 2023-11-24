use async_trait::async_trait;
use sea_orm::{ DatabaseConnection, Set, EntityTrait };

use crate::{
    database::embed_info::{ Entity as Embeds, ActiveModel as EmbedActiveModel },
    router::routes::RequestCreateUpdateEmbed,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};

pub struct MessageEmbedQueries {}

#[async_trait]
impl DefaultSeaQueries for MessageEmbedQueries {
    type Entity = Embeds;
    type ActiveModel = EmbedActiveModel;

    type CreateData = RequestCreateUpdateEmbed;
    type UpdateData = RequestCreateUpdateEmbed;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        Self::save_active_model(db, EmbedActiveModel {
            title: Set(create_data.title),
            url: Set(create_data.url),
            timestamp: Set(create_data.timestamp),
            color: Set(create_data.color),
            footer: Set(create_data.footer),
            image: Set(create_data.image),
            thumbnail: Set(create_data.thumbnail),
            author: Set(create_data.author),
            description: Set(create_data.description),
            footer_url: Set(create_data.footer_url),
            author_url: Set(create_data.author_url),
            ..Default::default()
        }).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        // Update the fields from the DTO
        if let Some(title) = update_data.title {
            active_model.title = Set(Some(title));
        }
        if let Some(url) = update_data.url {
            active_model.url = Set(Some(url));
        }
        if let Some(timestamp) = update_data.timestamp {
            active_model.timestamp = Set(Some(timestamp));
        }
        if let Some(color) = update_data.color {
            active_model.color = Set(Some(color));
        }
        if let Some(footer) = update_data.footer {
            active_model.footer = Set(Some(footer));
        }
        if let Some(image) = update_data.image {
            active_model.image = Set(Some(image));
        }
        if let Some(thumbnail) = update_data.thumbnail {
            active_model.thumbnail = Set(Some(thumbnail));
        }
        if let Some(author) = update_data.author {
            active_model.author = Set(Some(author));
        }
        if let Some(description) = update_data.description {
            active_model.description = Set(Some(description));
        }
        if let Some(footer_url) = update_data.footer_url {
            active_model.footer_url = Set(Some(footer_url));
        }
        if let Some(author_url) = update_data.author_url {
            active_model.author_url = Set(Some(author_url));
        }
        Ok(())
    }
}
