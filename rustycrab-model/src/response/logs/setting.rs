use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateLogSetting {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateLogSetting {
    pub specify_channels: Option<i8>,
    pub new_account_age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseLogSetting {
    pub id: i32,
    pub specify_channels: i8,
    pub new_account_age: i32,
    pub bot_id: i32,
    pub guild_id: i32,
}
