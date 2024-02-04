// src/queries/auth.rs

use crate::models::{ AuthResponse, AuthRequest };

pub async fn get_spotify_token(
    client_id: &str,
    client_secret: &str
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = AuthRequest {
        grant_type: "client_credentials".to_string(),
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
    };

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send().await?
        .json::<AuthResponse>().await?;

    Ok(res.access_token)
}
