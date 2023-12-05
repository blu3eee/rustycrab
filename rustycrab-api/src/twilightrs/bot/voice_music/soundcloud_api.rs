use headless_chrome::{ Browser, Tab };
// use serde_json::Value;
use std::{ error::Error, thread, time::Duration };

pub fn is_soundcloud_url(url: &str) -> bool {
    url.contains("soundcloud.com")
}

pub fn is_soundcloud_playlist_url(url: &str) -> bool {
    let url = url.to_lowercase(); // Normalize the URL for case-insensitive comparison

    // Check for traditional SoundCloud playlist URLs
    if
        (url.starts_with("https://soundcloud.com/") ||
            url.starts_with("http://soundcloud.com/") ||
            url.starts_with("https://www.soundcloud.com/") ||
            url.starts_with("http://www.soundcloud.com/")) &&
        url.contains("/sets/")
    {
        return true;
    }

    // Check for shortened SoundCloud playlist URLs
    if
        url.starts_with("https://on.soundcloud.com/") ||
        url.starts_with("http://on.soundcloud.com/") ||
        url.starts_with("http://www.on.soundcloud.com/") ||
        url.starts_with("https://www.on.soundcloud.com/")
    {
        // Extract the last segment of the URL
        if let Some(last_segment) = url.split('/').last() {
            // Check if the length of the last segment is exactly 5 characters
            return last_segment.len() == 5;
        }
    }

    false
}

pub async fn fetch_soundcloud_playlist_tracks(
    soundcloud_url: &str
) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    println!("fetch_soundcloud_playlist_tracks");
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(soundcloud_url)?;
    tab.wait_until_navigated()?; // Ensure the navigation is complete

    // Scroll and wait for content to load
    scroll_and_wait_for_content(&tab).await?;

    // Now extract the track URLs
    let tracks = extract_track_urls(&tab).await?;

    Ok(tracks)
}

async fn scroll_and_wait_for_content(tab: &Tab) -> Result<(), Box<dyn Error + Sync + Send>> {
    let scroll_script = "
        window.scrollTo(0, document.body.scrollHeight);
    ";

    // Scroll several times to ensure loading of all content
    for _ in 0..3 {
        tab.evaluate(scroll_script, false)?;
        // Wait for a moment to allow new content to load
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}

async fn extract_track_urls(tab: &Tab) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    let ul_selector = ".trackList__list.sc-clearfix.sc-list-nostyle";
    let element = tab.wait_for_element(ul_selector)?;

    let js_script =
        "
        function() {
            return Array.from(document.querySelectorAll('.trackList__list.sc-clearfix.sc-list-nostyle li.trackList__item')).map(li => {
                const linkElement = li.querySelector('a.trackItem__trackTitle');
                if (linkElement) {
                    return linkElement.getAttribute('href');
                }
            }).filter(href => href !== undefined);
        }
    ";

    let result = element.call_js_fn(js_script, vec![], false)?;

    let mut tracks: Vec<String> = Vec::new();
    if let Some(preview) = result.preview {
        for track in preview.properties {
            if let Some(url) = track.value {
                tracks.push(format!("https://soundcloud.com{}", url));
            }
        }
    }

    Ok(tracks)
}
