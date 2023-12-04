use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBotItem {
    pub id: i32,
    pub item_id: String,
    pub name: String,
    pub emoji: Option<String>,
    pub value: Option<i32>,
    pub functions: String,
    pub bot_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestCreateBotItem {
    pub bot_discord_id: String,
    pub item_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestUpdateBotItem {
    pub item_id: Option<String>,
    pub name: Option<String>,
    pub emoji: Option<String>,
    pub value: Option<i32>,
    pub functions: Option<Vec<String>>,
}
