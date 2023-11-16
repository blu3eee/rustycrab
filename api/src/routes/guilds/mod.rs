use serde::{ Deserialize, Serialize };
use crate::database::guild_info::Model as GuildModel;

#[derive(Serialize, Deserialize)]
pub struct ResponseGuild {
    pub id: i32,
    pub guild_id: String,
}

impl From<GuildModel> for ResponseGuild {
    fn from(model: GuildModel) -> Self {
        Self {
            id: model.id,
            guild_id: model.guild_id,
        }
    }
}

#[derive(Deserialize)]
pub struct RequestCreateGuild {
    pub guild_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateGuild {}
