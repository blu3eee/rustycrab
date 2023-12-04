use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionUserData {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}
