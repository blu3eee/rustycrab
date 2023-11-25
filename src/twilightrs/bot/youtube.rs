use std::{ env, error::Error };
use reqwest;
use serde::Deserialize;

// YouTube API response structure
#[derive(Deserialize)]
pub struct YoutubeSearchResponse {
    pub items: Vec<YoutubeVideoItem>,
}

#[derive(Deserialize)]
pub struct YoutubeVideoItem {
    pub id: YoutubeVideoId,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct YoutubeVideoId {
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
