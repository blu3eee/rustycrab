use axum::Json;
use rustycrab_model::auth::SessionUserData;
use tower_sessions::Session;

pub const SESSION_DATA_KEY: &str = "session_user";

pub async fn state(session: Session) -> Json<Option<SessionUserData>> {
    println!("state {:?}", session.get::<SessionUserData>(SESSION_DATA_KEY));

    let session_data = session.get::<SessionUserData>(SESSION_DATA_KEY).unwrap_or_default();

    Json(session_data)
}
