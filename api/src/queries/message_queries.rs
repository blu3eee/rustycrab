use async_trait::async_trait;
use sea_orm::{ DatabaseConnection, Set, EntityTrait, ActiveValue };

use crate::{
    database::{
        messages::{ Entity as Messages, ActiveModel as MessageActiveModel },
        embed_info::Model as EmbedModel,
    },
    router::routes::{ RequestCreateUpdateMessage, ResponseMessageDetails, ResponseMessage },
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
    twilightrs::messages::DiscordEmbed,
};

use super::message_embed_queries::MessageEmbedQueries;
pub struct MessageQueries {}

impl MessageQueries {
    pub async fn fetch_message_response(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<ResponseMessageDetails, AppError> {
        Ok(ResponseMessage::from(MessageQueries::find_by_id(db, id).await?).to_details(db).await?)
    }
}

#[async_trait]
impl DefaultSeaQueries for MessageQueries {
    type Entity = Messages;
    type ActiveModel = MessageActiveModel;
    type CreateData = RequestCreateUpdateMessage;
    type UpdateData = RequestCreateUpdateMessage;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        // First, handle the embed creation if it's present in the DTO
        let embed_model: Option<EmbedModel> = if let Some(embed_data) = create_data.embed {
            Some(MessageEmbedQueries::create_entity(db, embed_data).await?)
        } else {
            None
        };

        // Now, create the message itself
        let new_message: MessageActiveModel = MessageActiveModel {
            r#type: Set(create_data.r#type.unwrap_or_default()),
            content: Set(create_data.content),
            embed_id: Set(embed_model.as_ref().map(|e| e.id)), // Assuming there's an `embed_id` field linking to the embed
            ..Default::default() // Fill in other default values as necessary
        };

        // Insert the new message into the database
        Ok(Self::save_active_model(db, new_message).await?)

        // // Construct the response
        // let response: ResponseMessage = ResponseMessage {
        //     id: message.id,
        //     r#type: message.r#type, // Assuming `type` is wrapped in `Set`
        //     content: message.content, // Assuming `content` is wrapped in `Set`
        //     embed: embed_model.map(|e| { e.into() }),
        // };
    }

    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        println!("applying updates for message");
        if let Some(r#type) = update_data.r#type {
            active_model.r#type = Set(r#type);
        }
        if let Some(content) = update_data.content {
            active_model.content = Set(Some(content));
        }

        // Update embed if provided
        if let Some(embed_data) = update_data.embed {
            // Check if `message.embed_id` is `ActiveValue::Set(Some(id))`
            if let ActiveValue::Unchanged(Some(e_id)) = active_model.embed_id {
                println!("update embed {}", e_id);
                MessageEmbedQueries::update_by_id(db, e_id, embed_data).await?;
            } else {
                MessageEmbedQueries::create_entity(db, embed_data).await?;
            }
        }
        Ok(())
    }
}

pub async fn create_embed(
    db: &DatabaseConnection,
    message_id: Option<i32>
) -> Result<DiscordEmbed, AppError> {
    let message_id = message_id.ok_or_else(|| AppError::bad_request("Message ID not found"))?;
    let message = MessageQueries::find_by_id(db, message_id).await?;
    let embed_id = message.embed_id.ok_or_else(|| AppError::bad_request("Embed ID not found"))?;

    let embed: DiscordEmbed = MessageEmbedQueries::find_by_id(db, embed_id).await?.into();

    Ok(embed)
}
