pub mod get_one;

use serde::{ Deserialize, Serialize };

use crate::database::bot_users::Model as BotUserModel;

#[derive(Deserialize)]
pub struct RequestCreateBotUser {
    pub bot_id: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct RequestUpdateBotUser {
    pub balance: Option<i32>,
    pub pray_points: Option<i32>,
    pub inventory: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBotUser {
    pub id: i32,
    pub balance: i32,
    pub pray_points: i32,
    pub inventory: String,
    pub bot_id: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBotUser {
    pub data: ResponseBotUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataBotUsers {
    pub data: Vec<ResponseBotUser>,
}

impl From<BotUserModel> for ResponseBotUser {
    fn from(model: BotUserModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            user_id: model.user_id,
            balance: model.balance,
            pray_points: model.pray_points,
            inventory: model.inventory,
        }
    }
}
