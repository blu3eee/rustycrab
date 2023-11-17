use serde::{ Deserialize, Serialize };

use crate::database::users::Model as UserModel;

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub discord_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateUser {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub discord_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

impl From<UserModel> for ResponseUser {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            discord_id: model.discord_id,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUsers {
    pub data: Vec<ResponseUser>,
}

// Function to convert from SeaORM Model to you
