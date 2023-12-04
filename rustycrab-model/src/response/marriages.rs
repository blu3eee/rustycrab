use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMarriageRelationship {
    pub id: i32,
    pub date_of_marriage: i32,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
    pub quote: Option<String>,
    pub partner1_id: i32,
    pub partner2_id: i32,
    pub ring_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestCreateMarriage {
    pub bot_discord_id: String,
    pub user1_discord_id: String,
    pub user2_discord_id: String,
    pub date_of_marriage: i32,
    pub ring_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestUpdateMarriage {
    pub date_of_marriage: Option<i32>,
    pub ring_id: Option<i32>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub caption: Option<String>,
    pub quote: Option<String>,
}
