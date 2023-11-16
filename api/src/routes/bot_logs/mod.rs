use crate::database::log_settings::Model as LogSettingModel;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct RequestCreateLogSetting {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateLogSetting {
    pub specify_channels: Option<i8>,
    pub new_account_age: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseLogSetting {
    pub id: i32,
    pub specify_channels: i8,
    pub new_account_age: i32,
    pub bot_id: Option<i32>,
    pub guild_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataLogSetting {
    pub data: ResponseLogSetting,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataLogSettings {
    pub data: Vec<ResponseLogSetting>,
}

impl From<LogSettingModel> for ResponseLogSetting {
    fn from(model: LogSettingModel) -> Self {
        Self {
            id: model.id,
            specify_channels: model.specify_channels,
            new_account_age: model.new_account_age,
            bot_id: model.bot_id,
            guild_id: model.guild_id,
        }
    }
}
