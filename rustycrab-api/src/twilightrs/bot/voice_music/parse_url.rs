use futures_util::future::join_all;
use songbird::input::{ YoutubeDl, Compose };

use crate::{ twilightrs::discord_client::DiscordClient, utilities::app_error::BoxedError };

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
) -> Result<(Vec<String>, Vec<String>), BoxedError> {
    let urls = if url.starts_with("http://") || url.starts_with("https://") {
        if is_youtube_url(url) || is_soundcloud_url(url) {
            if is_youtube_playlist_url(url) {
                match fetch_playlist_videos(url).await {
                    Ok(urls) => { urls }
                    Err(_) => {
                        return Err(
                            client
                                .get_locale_string(locale, "music-playlist-fetch-error", None)
                                .into()
                        );
                    }
                }
            } else if is_soundcloud_playlist_url(url) {
                match fetch_soundcloud_playlist_tracks(url).await {
                    Ok(urls) => { urls }
                    Err(_) => {
                        return Err(
                            client
                                .get_locale_string(locale, "music-playlist-fetch-error", None)
                                .into()
                        );
                    }
                }
            } else {
                vec![url.to_string()]
            }
        } else {
            return Err(client.get_locale_string(locale, "music-note", None).into());
        }
    } else {
        // Perform a YouTube search and get the first result's URL
        match search_youtube(url).await {
            Ok(url) => vec![url],
            Err(_) => {
                return Err(client.get_locale_string(locale, "music-search-error", None).into());
            }
        }
    };
    // println!("urls {:?}", urls);
    Ok(prune_list(urls).await)
}

async fn check_url(url: &str) -> Result<(String, String), BoxedError> {
    let mut source = YoutubeDl::new(reqwest::Client::new(), url.to_string());
    match source.aux_metadata().await {
        Ok(metadata) => {
            Ok((
                url.to_string(),
                format!(
                    "[{}]({})",
                    metadata.title.unwrap_or_else(|| "Unknown title".to_string()),
                    url
                ),
            ))
        }
        Err(_) => { Err("invalid track".into()) }
    }
}

async fn prune_list(urls: Vec<String>) -> (Vec<String>, Vec<String>) {
    let tasks = urls.iter().map(|url| {
        let url_cloned = url.clone();
        tokio::spawn(async move { check_url(&url_cloned).await })
    });

    let mut valid_urls = Vec::new();
    let mut results = Vec::new();

    for task in join_all(tasks).await {
        match task {
            Ok(result) => {
                match result {
                    Ok(result) => {
                        valid_urls.push(result.0);
                        results.push(result.1);
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    // println!("pruned: {:?}\n{:?}", valid_urls, results);
    (valid_urls, results)
}
