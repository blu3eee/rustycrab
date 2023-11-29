use sea_orm::DatabaseConnection;
use serde::{ Serialize, Deserialize };

use crate::{
    database::auto_responses::Model as AutoResModel,
    queries::{
        message_queries::MessageQueries,
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
    },
    default_queries::DefaultSeaQueries,
    utilities::app_error::AppError,
};

use super::{
    RequestCreateUpdateMessage,
    ResponseMessageDetails,
    guilds::ResponseGuild,
    bots::ResponseBot,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAutoRes {
    pub id: i32,
    pub trigger: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub response_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAutoResDetails {
    pub id: i32,
    pub trigger: String,
    pub bot: ResponseBot,
    pub guild: ResponseGuild,
    pub response: Option<ResponseMessageDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateAutoResponse {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub trigger: String,
    pub response_data: Option<RequestCreateUpdateMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestUpdateAutoResponse {
    pub trigger: Option<String>,
    pub response_data: Option<RequestCreateUpdateMessage>,
}

impl From<AutoResModel> for ResponseAutoRes {
    fn from(model: AutoResModel) -> Self {
        Self {
            id: model.id,
            trigger: model.trigger,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            response_id: model.response_id,
        }
    }
}

impl ResponseAutoRes {
    pub async fn to_details(
        &self,
        db: &DatabaseConnection
    ) -> Result<ResponseAutoResDetails, AppError> {
        let bot = BotQueries::find_by_id(db, self.bot_id).await?;
        let guild = GuildQueries::find_by_id(db, self.guild_id).await?;
        let response = if let Some(id) = self.response_id {
            Some(MessageQueries::fetch_message_response(db, id).await?)
        } else {
            None
        };

        Ok(ResponseAutoResDetails {
            id: self.id,
            trigger: self.trigger.clone(),
            bot: bot.into(),
            guild: guild.into(),
            response,
        })
    }
}
