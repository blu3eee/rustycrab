use twilight_model::id::{ marker::UserMarker, Id };

pub struct UserAfkStatus {
    pub message: Option<String>,
    pub since: u32,
    pub activities_count: u8,
    pub notify: Vec<Id<UserMarker>>,
}

impl UserAfkStatus {
    pub fn new(message: Option<String>, since: u32) -> Self {
        Self {
            message,
            since,
            activities_count: 0,
            notify: vec![],
        }
    }
}
