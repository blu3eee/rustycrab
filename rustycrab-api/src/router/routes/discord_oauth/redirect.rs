use std::time::SystemTime;

use axum::{ Extension, extract::Query, response::Redirect };
use oauth2::{ basic::BasicClient, AuthorizationCode, reqwest::async_http_client, TokenResponse };
use rustycrab_model::auth::SessionUserData;
use serde::Deserialize;
use time::Duration;
use tower_sessions::{ Session, Expiry };

#[derive(Deserialize)]
pub struct DiscordCallbackQuery {
    code: String,
}

use crate::router::routes::discord_oauth::auth::fetch_discord_user_info;

use super::state::SESSION_DATA_KEY;

pub async fn redirect(
    session: Session,
    Extension(client): Extension<BasicClient>,
    Query(query): Query<DiscordCallbackQuery>
) -> Redirect {
    println!("session {:?}", session.get::<SessionUserData>(SESSION_DATA_KEY));
    let token_res = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client).await;

    match token_res {
        Ok(token_response) => {
            let access_token = token_response.access_token().secret().to_string();
            let refresh_token = if let Some(token) = token_response.refresh_token() {
                Some(token.secret().to_string())
            } else {
                None
            };

            if let Some(expires_in) = token_response.expires_in() {
                let expiration_time = SystemTime::now() + expires_in;
                session.set_expiry(Some(Expiry::AtDateTime(expiration_time.into())));
            } else {
                session.set_expiry(Some(Expiry::OnInactivity(Duration::seconds(60 * 60))));
            }

            let user = fetch_discord_user_info(&access_token).await;
            match user {
                Ok(user) => {
                    let insert_result = session.insert(SESSION_DATA_KEY, SessionUserData {
                        user_id: user.id.clone(),
                        access_token,
                        refresh_token,
                    });

                    println!("session insert_result {insert_result:?}");

                    println!("session {:?}", session.get::<SessionUserData>(SESSION_DATA_KEY));
                    Redirect::to(&format!("http://localhost:8000/about"))
                }
                Err(err) => {
                    println!("error fetching user from discord api: {err:?}");
                    Redirect::to("http://localhost:8000")
                }
            }
        }
        Err(_) => Redirect::to("http://localhost:8000/login/failed"),
    }
}
