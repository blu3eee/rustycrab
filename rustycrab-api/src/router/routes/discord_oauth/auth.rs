// use reqwest::Client as HttpClient;
use serde::Deserialize;
use oauth2::{ AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, basic::BasicClient };
use std::env;

// Initialize OAuth2 client
pub fn discord_oauth_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(
            env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set in the .env file")
        ),
        Some(
            ClientSecret::new(
                env
                    ::var("DISCORD_CLIENT_SECRET")
                    .expect("DISCORD_CLIENT_SECRET must be set in the .env file")
            )
        ),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap())
    ).set_redirect_uri(
        RedirectUrl::new("http://localhost:8080/api/auth/redirect".to_string()).unwrap()
    )
}

#[derive(Deserialize, Debug)]
pub struct DiscordUser {
    pub username: String,
    pub id: String,
}

pub async fn fetch_discord_user_info(access_token: &str) -> Result<DiscordUser, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://discord.com/api/v10/users/@me")
        .bearer_auth(format!("{access_token}"))
        .send().await?
        .json::<DiscordUser>().await?;

    Ok(res)
}
