use serde::{ Deserialize, Serialize };
use crate::database::guild_info::Model as GuildModel;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateGuild {
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateGuild {}
