pub mod hello_world;

pub mod users_routes;
pub mod bots;
pub mod guilds;
pub mod bot_guild_configs;
pub mod bot_users;
pub mod bot_guild_welcomes;
pub mod bot_logs;
pub mod tickets;
pub mod auto_responses;
pub mod discord_oauth;

use crate::database::{ embed_info::Model as EmbedModel, buttons::Model as ButtonModel };
use rustycrab_model::response::discord_message::{ ResponseEmbed, ResponseButton };

// impl ResponseMessage {
//     pub async fn to_details(
//         &self,
//         db: &DatabaseConnection
//     ) -> Result<ResponseMessageDetails, AppError> {
//         let embed = if let Some(e_id) = self.embed_id {
//             let embed_model = MessageEmbedQueries::find_by_id(db, e_id).await?;
//             Some(ResponseEmbed::from(embed_model)) // Assuming `From` trait is implemented for `ResponseEmbed`
//         } else {
//             None
//         };

//         Ok(ResponseMessageDetails {
//             id: self.id,
//             r#type: self.r#type.clone(),
//             content: self.content.clone(),
//             embed,
//         })
//     }
// }

impl From<EmbedModel> for ResponseEmbed {
    fn from(model: EmbedModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            url: model.url,
            timestamp: model.timestamp,
            color: model.color,
            footer: model.footer,
            image: model.image,
            thumbnail: model.thumbnail,
            author: model.author,
            description: model.description,
            footer_url: model.footer_url,
            author_url: model.author_url,
        }
    }
}

impl From<ButtonModel> for ResponseButton {
    fn from(model: ButtonModel) -> Self {
        Self {
            id: model.id,
            text: model.text,
            color: model.color,
            emoji: model.emoji,
        }
    }
}
