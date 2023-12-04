use axum::{
    Router,
    routing::get,
    Extension,
    error_handling::HandleErrorLayer,
    BoxError,
    http::StatusCode,
};
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{ MemoryStore, SessionManagerLayer, Expiry };

mod login;
mod auth;
mod redirect;
mod state;

pub async fn auth_routes() -> Router {
    let oauth_client = auth::discord_oauth_client();
    let session_store = {
        println!("initialize session store");
        MemoryStore::default()
    };
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async { StatusCode::BAD_REQUEST }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 60)))
        );

    Router::new().nest(
        "/auth",
        Router::new()
            .route("/login", get(login::login))
            .route("/redirect", get(redirect::redirect))
            .route("/state", get(state::state))
            .layer(session_service)
            .layer(Extension(oauth_client))
    )
}
