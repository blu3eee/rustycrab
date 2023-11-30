use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateLogAction {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub channel_id: String,
    pub events: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateActionLog {
    pub channel_id: Option<String>,
    pub events: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseActionLog {
    pub id: i32,
    pub channel_id: String,
    pub bot_id: i32,
    pub guild_id: i32,
    pub events: String,
}
