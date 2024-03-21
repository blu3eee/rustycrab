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
            return last_segment.len() == 17;
        }
    }

    false
}
