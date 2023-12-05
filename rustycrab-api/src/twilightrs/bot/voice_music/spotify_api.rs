use regex::Regex;

pub fn is_spotify_link(url: &str) -> bool {
    let re = Regex::new(r"(https?://)?(open\.)?spotify\.com/(track|playlist)/").unwrap();
    re.is_match(url) || url.contains("spotify.link")
}
