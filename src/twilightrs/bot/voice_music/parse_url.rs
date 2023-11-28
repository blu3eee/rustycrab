use std::error::Error;

use crate::twilightrs::discord_client::DiscordClient;

use super::{
    youtube_api::{ is_youtube_url, is_youtube_playlist_url, fetch_playlist_videos, search_youtube },
    soundcloud_api::{
        is_soundcloud_url,
        fetch_soundcloud_playlist_tracks,
        is_soundcloud_playlist_url,
    },
};

pub async fn parse_url_or_search_query(
    client: &DiscordClient,
    locale: &str,
    url: &str
) -> Result<Vec<String>, Box<dyn Error + Send + Sync + 'static>> {
    if url.starts_with("http://") || url.starts_with("https://") {
        if is_youtube_url(url) || is_soundcloud_url(url) {
            if is_youtube_playlist_url(url) {
                match fetch_playlist_videos(url).await {
                    Ok(urls) => Ok(urls),
                    Err(_) => {
                        Err(
                            client
                                .get_locale_string(locale, "music-playlist-fetch-error", None)
                                .into()
                        )
                    }
                }
            } else if is_soundcloud_playlist_url(url) {
                match fetch_soundcloud_playlist_tracks(url).await {
                    Ok(urls) => Ok(urls),
                    Err(_) => {
                        Err(
                            client
                                .get_locale_string(locale, "music-playlist-fetch-error", None)
                                .into()
                        )
                    }
                }
            } else {
                Ok(vec![url.to_string()])
            }
        } else {
            Err(client.get_locale_string(locale, "music-note", None).into())
        }
    } else {
        // Perform a YouTube search and get the first result's URL
        match search_youtube(url).await {
            Ok(url) => Ok(vec![url]),
            Err(_) => { Err(client.get_locale_string(locale, "music-search-error", None).into()) }
        }
    }
}
