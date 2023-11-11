use crate::database::bot_guild_welcomes::Model as WelcomeModel;

use serde::{ Deserialize, Serialize };

use super::{
    RequestCreateUpdateMessage,
    bots::ResponseBot,
    guilds::ResponseGuild,
    ResponseMessage,
};

#[derive(Deserialize)]
pub struct RequestCreateWelcome {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub message_data: Option<RequestCreateUpdateMessage>,
    pub channel_id: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestUpdateWelcome {
    pub channel_id: Option<String>,
    pub message_data: Option<RequestCreateUpdateMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseGuildWelcome {
    pub id: i32,
    pub enabled: i8,
    pub channel_id: Option<String>,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
    pub message_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseGuildWelcomeDetails {
    pub id: i32,
    pub enabled: i8,
    pub channel_id: Option<String>,
    pub bot: Option<ResponseBot>,
    pub guild: Option<ResponseGuild>,
    pub message: Option<ResponseMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataGuildWelcome {
    pub data: ResponseGuildWelcome,
}

impl From<WelcomeModel> for ResponseGuildWelcome {
    fn from(model: WelcomeModel) -> Self {
        Self {
            id: model.id,
            enabled: model.enabled,
            channel_id: model.channel_id,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
            message_id: model.message_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataGuildWelcomes {
    pub data: Vec<ResponseGuildWelcome>,
}

// Function to convert from SeaORM Model to you
