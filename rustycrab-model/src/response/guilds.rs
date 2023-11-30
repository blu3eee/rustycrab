use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseGuild {
    pub id: i32,
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateGuild {
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateGuild {}
