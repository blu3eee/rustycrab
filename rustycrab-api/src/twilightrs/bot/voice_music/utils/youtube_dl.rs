use rustycrab_model::error::BoxedError;
use youtube_dl::YoutubeDl as YouTubeDlSearch;

use url::Url;

pub fn is_youtube_url(url: &str) -> bool {
    url.contains("youtube.com") || url.contains("youtu.be/")
}

pub fn is_youtube_playlist_url(url: &str) -> bool {
    let parsed_url = Url::parse(url).unwrap();
    parsed_url.query_pairs().any(|(key, _)| key == "list")
}

pub async fn get_youtube_playlist_tracks(url: &str) -> Result<Vec<String>, BoxedError> {
    let output = YouTubeDlSearch::new(url)
        .socket_timeout("15")
        .flat_playlist(true)
        .ignore_errors(true)
        .run();
    match output {
        Ok(output) => {
            Ok(
                output
                    .into_playlist()
                    .ok_or("can't find playlist")?
                    .entries.ok_or("no tracks found")?
                    .iter()
                    .filter_map(|video| video.url.clone())
                    .collect::<Vec<String>>()
            )
        }
        Err(e) => {
            eprintln!("error youtubedlsearch: {e:}");
            return Err(e.into());
        }
    }
}
