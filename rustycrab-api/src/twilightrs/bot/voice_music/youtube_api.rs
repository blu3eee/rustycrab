use std::{ env, error::Error };
use reqwest;
use serde::Deserialize;

// YouTube API response structure
#[derive(Deserialize, Debug)]
pub struct YoutubeSearchResponse {
    pub items: Vec<YoutubeVideoSnippet>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct YoutubeVideoSnippet {
    pub id: YoutubeVideoDetails,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PlaylistYoutubeVideoItem {
    pub contentDetails: YoutubeVideoDetails,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct YoutubeVideoDetails {
    pub videoId: String,
}

pub async fn search_youtube(query: &str) -> Result<String, Box<dyn Error + Sync + Send>> {
    let api_key: String = env
        ::var("YOUTUBE_API")
        .expect("YOUTUBE_API must be set in the .env file");
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=video&key={}",
        query,
        api_key
    );

    let resp = reqwest::get(&url).await?.json::<YoutubeSearchResponse>().await?;

    if let Some(first_item) = resp.items.get(0) {
        Ok(format!("https://www.youtube.com/watch?v={}", first_item.id.videoId))
    } else {
        Err("No results found".into())
    }
}

use url::Url;

pub fn is_youtube_url(url: &str) -> bool {
    url.contains("youtube.com")
}

pub fn is_youtube_playlist_url(url: &str) -> bool {
    let parsed_url = Url::parse(url).unwrap();
    parsed_url.query_pairs().any(|(key, _)| key == "list")
}

fn extract_playlist_id(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    parsed_url
        .query_pairs()
        .find(|(key, _)| key == "list")
        .map(|(_, value)| value.to_string())
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct YoutubePlaylistResponse {
    pub items: Vec<PlaylistYoutubeVideoItem>,
}

pub async fn fetch_playlist_videos(url: &str) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    let api_key: String = env
        ::var("YOUTUBE_API")
        .expect("YOUTUBE_API must be set in the .env file");

    let playlist_id = extract_playlist_id(url).ok_or("Invalid playlist URL")?;
    let base_url = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=contentDetails&playlistId={}&maxResults=50&key={}",
        playlist_id,
        api_key
    );

    let mut video_urls = Vec::new();

    let resp = reqwest::get(&base_url).await?.json::<YoutubePlaylistResponse>().await?;

    for item in &resp.items {
        video_urls.push(format!("https://www.youtube.com/watch?v={}", item.contentDetails.videoId));
    }

    Ok(video_urls)
}
