use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateUser {
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateUser {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: i32,
    pub discord_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
