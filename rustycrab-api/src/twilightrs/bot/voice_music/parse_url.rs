use futures_util::future::join_all;
use songbird::input::{ YoutubeDl, Compose };
use spotify_api::queries::{
    extract_spotify_id_from_url,
    track::get_track_data,
    playlist::get_playlist_data,
};

use youtube_dl::YoutubeDl as YouTubeDlSearch;

use crate::{ twilightrs::discord_client::DiscordClient, utilities::app_error::BoxedError };

use super::{
    youtube_api::{ is_youtube_url, is_youtube_playlist_url },
    soundcloud_api::{ is_soundcloud_url, is_soundcloud_playlist_url },
    spotify_api::is_spotify_link,
};

pub async fn parse_url_or_search_query(
    client: &DiscordClient,
    locale: &str,
    url: &str
) -> Result<(Vec<String>, Vec<String>), BoxedError> {
    let urls = if url.starts_with("http://") || url.starts_with("https://") {
        println!("{url} {}", is_spotify_link(url));
        if is_youtube_url(url) || is_soundcloud_url(url) {
            if is_youtube_playlist_url(url) {
                println!("fetching youtube playlist");
                let output = YouTubeDlSearch::new(url)
                    .socket_timeout("15")
                    .flat_playlist(true)
                    .ignore_errors(true)
                    .run();
                println!("output {output:?}");
                match output {
                    Ok(output) => {
                        output
                            .into_playlist()
                            .ok_or("can't find playlist")?
                            .entries.ok_or("no tracks found")?
                            .iter()
                            .filter_map(|video| video.url.clone())
                            .collect::<Vec<String>>()
                    }
                    Err(e) => {
                        eprintln!("error youtubedlsearch: {e:}");
                        return Err(e.into());
                    }
                }
            } else if is_soundcloud_playlist_url(url) {
                let output = YouTubeDlSearch::new(url)
                    .socket_timeout("15")
                    .flat_playlist(true)
                    .ignore_errors(true)
                    .run();
                match output {
                    Ok(output) => {
                        output
                            .into_playlist()
                            .ok_or("can't find playlist")?
                            .entries.ok_or("no tracks found")?
                            .iter()
                            .filter_map(|video| video.url.clone())
                            .collect::<Vec<String>>()
                    }
                    Err(e) => {
                        eprintln!("error youtubedlsearch: {e:}");
                        return Err(e.into());
                    }
                }
            } else {
                vec![url.to_string()]
            }
        } else if is_spotify_link(url) {
            let spotify_tracks = if let Some((url_type, id)) = extract_spotify_id_from_url(url) {
                if url_type == "track" {
                    println!("spotify track");
                    let track_data = get_track_data(&id).await?;
                    vec![
                        format!(
                            "{} {}",
                            track_data.name,
                            track_data.artists
                                .iter()
                                .map(|artist| artist.name.clone())
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                    ]
                } else {
                    println!("spotify playlist");
                    let playlist_data = get_playlist_data(&id).await?;
                    playlist_data.tracks.items
                        .iter()
                        .map(|track_item| {
                            let track_data = &track_item.track;
                            format!(
                                "{} {}",
                                track_data.name,
                                track_data.artists
                                    .iter()
                                    .map(|artist| artist.name.clone())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            )
                        })
                        .collect::<Vec<String>>()
                }
            } else {
                return Err(
                    client.get_locale_string(locale, "music-playlist-fetch-error", None).into()
                );
            };
            spotify_tracks
                .iter()
                .map(|track| format!("ytsearch1:{}", track))
                .collect::<Vec<String>>()
        } else {
            return Err(client.get_locale_string(locale, "music-note", None).into());
        }
    } else {
        vec![format!("ytsearch1:{url}")]
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
