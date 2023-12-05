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

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyTrackResponse {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub images: Vec<Image>,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyPlaylistResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tracks: PlaylistTracks,
    pub owner: User,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTracks {
    pub items: Vec<PlaylistTrackItem>,
    pub total: u32,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrackItem {
    pub track: SpotifyTrackResponse,
    // include other fields as needed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub display_name: Option<String>,
    // include other fields as needed
}
