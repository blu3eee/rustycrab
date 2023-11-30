use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateBotUser {
    pub bot_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateBotUser {
    pub balance: Option<i32>,
    pub pray_points: Option<i32>,
    pub inventory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBotUser {
    pub id: i32,
    pub balance: i32,
    pub pray_points: i32,
    pub inventory: String,
    pub bot_id: i32,
    pub user_id: i32,
}
