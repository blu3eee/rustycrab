use spotify::{
    queries::{ extract_spotify_id_from_url, track::get_track_data, playlist::get_playlist_data },
    models::SpotifyPlaylistResponse,
};

use crate::{ twilightrs::discord_client::DiscordClient, utilities::app_error::BoxedError };

use super::{
    youtube_dl::{ is_youtube_url, is_youtube_playlist_url, get_youtube_playlist_tracks },
    soundcloud::{ is_soundcloud_url, is_soundcloud_playlist_url },
    spotify::is_spotify_link,
};

pub async fn parse_url_or_search_query(
    client: &DiscordClient,
    locale: &str,
    url: &str
) -> Result<(Vec<String>, Option<SpotifyPlaylistResponse>), BoxedError> {
    let mut spotify_playlist: Option<SpotifyPlaylistResponse> = None;
    let urls = if url.starts_with("http://") || url.starts_with("https://") {
        if is_youtube_url(url) || is_soundcloud_url(url) {
            if is_youtube_playlist_url(url) || is_soundcloud_playlist_url(url) {
                get_youtube_playlist_tracks(url).await?
            } else {
                vec![url.to_string()]
            }
        } else if is_spotify_link(url) {
            let spotify_tracks = if let Some((url_type, id)) = extract_spotify_id_from_url(url) {
                if url_type == "track" {
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
                    let playlist_data = get_playlist_data(&id).await?;
                    spotify_playlist = Some(playlist_data.clone());
                    playlist_data.tracks.items
                        .iter()
                        .map(|track_item| {
                            let track_data = &track_item.track;
                            format!(
                                "ytsearch1:{} {} official",
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
        } else {
            return Err(client.get_locale_string(locale, "music-note", None).into());
        }
    } else {
        vec![format!("ytsearch1:{url}")]
    };

    Ok((urls.clone(), spotify_playlist))
}
