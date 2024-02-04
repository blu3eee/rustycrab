// src/queries/mod.rs

use std::env;

use regex::Regex;
use reqwest;
use serde::de::DeserializeOwned;

use self::auth::get_spotify_token;

pub mod auth;
pub mod track;
pub mod playlist;
pub mod manager;

pub async fn get_spotify_data<T>(url: &str) -> Result<T, reqwest::Error> where T: DeserializeOwned {
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Expected a client id");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("Expected a client secret");

    let token = get_spotify_token(&client_id, &client_secret).await?;
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send().await?
        .json::<T>().await?;

    Ok(res)
}

pub async fn get_final_spotify_url(short_url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(short_url).await?;
    Ok(resp.url().to_string())
}

pub fn extract_spotify_id_from_url(url: &str) -> Option<(String, String)> {
    let re = Regex::new(r"spotify\.com/(playlist|track)/([a-zA-Z0-9]+)").unwrap();
    re.captures(url).and_then(|caps| {
        let kind = caps.get(1)?.as_str().to_string();
        let id = caps.get(2)?.as_str().to_string();
        Some((kind, id))
    })
}
