use std::error::Error;
// use scraper::{ Html, Selector };
// use fantoccini::{ ClientBuilder, Locator };
// use scraper::{ Html, Selector };

pub fn is_soundcloud_url(url: &str) -> bool {
    url.contains("soundcloud.com")
}

pub fn is_soundcloud_playlist_url(url: &str) -> bool {
    url.contains("soundcloud.com") && url.contains("/sets/")
}

pub async fn fetch_soundcloud_playlist_tracks(
    soundcloud_url: &str
) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    println!("fetch_soundcloud_playlist_tracks");
    let _ = reqwest::get(soundcloud_url).await?.text().await?;

    // let fragment = Html::parse_fragment(&html);
    // let document = Html::parse_fragment(&html);

    // println!("{:?}", document);
    // Corrected CSS selector
    // let track_selector = Selector::parse(r#"a[itemprop=\"url\"]"#).unwrap();
    // println!("{:?}", document.select(&track_selector));
    // println!("{:?}", document.select(&Selector::parse("section.trackList").unwrap()));
    // println!("{:?}", document.select(&Selector::parse("a.trackitem__tracktitle").unwrap()));
    let track_urls = Vec::new();
    // for track in document.select(&track_selector) {
    //     println!("{:?}", track);
    //     if let Some(track_url) = track.value().attr("href") {
    //         println!("track_url");
    //         // Construct the full track URL and add it to the vector
    //         track_urls.push(format!("https://soundcloud.com{}", track_url));
    //     }
    // }

    Ok(track_urls)
}
