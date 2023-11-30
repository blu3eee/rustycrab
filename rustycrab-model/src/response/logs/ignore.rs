use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAllGuildIgnores {
    channels: Vec<ResponseLogIgnoreChannel>,
    roles: Vec<ResponseLogIgnoreRole>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateLogIgnoreChannel {
    pub log_setting_id: i32,
    pub channel_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateLogIgnoreChannel {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseLogIgnoreChannel {
    pub id: i32,
    pub log_setting_id: i32,
    pub channel_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateLogIgnoreRole {
    pub log_setting_id: i32,
    pub role_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdateLogIgnoreRole {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseLogIgnoreRole {
    pub id: i32,
    pub log_setting_id: i32,
    pub role_id: String,
}
