use sea_orm::{ DatabaseConnection, Set, ColumnTrait, EntityTrait, QueryFilter };

use crate::{
    database::embed_info::{
        self,
        Entity as Embeds,
        Model as EmbedModel,
        ActiveModel as EmbedActiveModel,
    },
    routes::RequestCreateUpdateEmbed,
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
};

use super::save_active_model;

pub async fn create_embed(
    db: &DatabaseConnection,
    create_dto: RequestCreateUpdateEmbed
) -> Result<EmbedModel, AppError> {
    save_active_model(db, EmbedActiveModel {
        title: Set(create_dto.title),
        url: Set(create_dto.url),
        timestamp: Set(create_dto.timestamp),
        color: Set(create_dto.color),
        footer: Set(create_dto.footer),
        image: Set(create_dto.image),
        thumbnail: Set(create_dto.thumbnail),
        author: Set(create_dto.author),
        description: Set(create_dto.description),
        footer_url: Set(create_dto.footer_url),
        author_url: Set(create_dto.author_url),
        ..Default::default()
    }).await
}

pub async fn update_embed(
    db: &DatabaseConnection,
    id: &i32,
    update_dto: RequestCreateUpdateEmbed
) -> Result<EmbedModel, AppError> {
    let mut active_model: EmbedActiveModel = Embeds::find()
        .filter(embed_info::Column::Id.eq(*id))
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Embed not found"))?
        .into();

    // Update the fields from the DTO
    if let Some(title) = update_dto.title {
        active_model.title = Set(Some(title));
    }
    if let Some(url) = update_dto.url {
        active_model.url = Set(Some(url));
    }
    if let Some(timestamp) = update_dto.timestamp {
        active_model.timestamp = Set(Some(timestamp));
    }
    if let Some(color) = update_dto.color {
        active_model.color = Set(Some(color));
    }
    if let Some(footer) = update_dto.footer {
        active_model.footer = Set(Some(footer));
    }
    if let Some(image) = update_dto.image {
        active_model.image = Set(Some(image));
    }
    if let Some(thumbnail) = update_dto.thumbnail {
        active_model.thumbnail = Set(Some(thumbnail));
    }
    if let Some(author) = update_dto.author {
        active_model.author = Set(Some(author));
    }
    if let Some(description) = update_dto.description {
        active_model.description = Set(Some(description));
    }
    if let Some(footer_url) = update_dto.footer_url {
        active_model.footer_url = Set(Some(footer_url));
    }
    if let Some(author_url) = update_dto.author_url {
        active_model.author_url = Set(Some(author_url));
    }

    save_active_model(db, active_model).await
}

pub async fn get_embed(db: &DatabaseConnection, id: &i32) -> Result<EmbedModel, AppError> {
    Embeds::find()
        .filter(embed_info::Column::Id.eq(*id))
        .one(db).await
        .map_err(convert_seaorm_error)
        .and_then(|embed| embed.ok_or_else(|| AppError::not_found("Embed not found")))
}

pub async fn delete_embed(db: &DatabaseConnection, id: &i32) -> Result<(), AppError> {
    Embeds::delete_by_id(*id)
        .exec(db).await
        .map_err(|err| {
            eprintln!("Error deleting embed with id {}: {:?}", id, err);
            AppError::internal_server_error("There was an error deleting the embed")
        })?;

    Ok(())
}
