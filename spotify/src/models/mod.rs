use std::error::Error;

use serde::{ Deserialize, Serialize };

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Serialize)]
pub struct AuthRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpotifyTrackResponse {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub duration_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub images: Vec<Image>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpotifyPlaylistResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tracks: PlaylistTracks,
    pub owner: User,
    pub images: Vec<Image>,
    pub followers: PlaylistFollowers,
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistTracks {
    pub items: Vec<PlaylistTrackItem>,
    pub total: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistTrackItem {
    pub track: SpotifyTrackResponse,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistFollowers {
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub display_name: Option<String>,
    pub external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExternalUrls {
    pub spotify: String,
}
