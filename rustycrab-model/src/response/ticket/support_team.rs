use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateTicketSupportTeam {
    pub bot_discord_id: String,
    pub guild_discord_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateTicketSupportTeam {
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub users: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTicketSupportTeam {
    pub id: i32,
    pub bot_id: i32,
    pub guild_id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub users: Vec<String>,
}
