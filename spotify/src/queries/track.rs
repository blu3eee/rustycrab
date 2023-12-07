use crate::models::SpotifyTrackResponse;

use super::get_spotify_data;

pub async fn get_track_data(id: &str) -> Result<SpotifyTrackResponse, reqwest::Error> {
    let url = format!("https://api.spotify.com/v1/tracks/{id}"); // Replace with the actual API endpoint
    get_spotify_data(&url).await
}
