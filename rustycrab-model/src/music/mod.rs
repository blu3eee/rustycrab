use serde::Deserialize;
use std::str::FromStr;

use enum_primitive_derive::Primitive;
use num_traits::{ FromPrimitive, ToPrimitive };

#[derive(Clone, Debug)]
pub enum PlayerLoopState {
    NoLoop,
    LoopCurrentTrack,
    LoopQueue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum MusicPlayerActions {
    Pause = 0,
    Clear = 1,
    Next = 2,
    LoopQueue = 3,
    LoopTrack = 4,
}

impl FromStr for MusicPlayerActions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(num) => MusicPlayerActions::from_i32(num).ok_or(()),
            Err(_) => Err(()),
        }
    }
}

impl MusicPlayerActions {
    pub fn to_i32_string(&self) -> String {
        self.to_i32().unwrap().to_string()
    }
}

// YouTube API response structure
#[derive(Deserialize, Debug)]
pub struct YoutubeSearchResponse {
    pub items: Vec<YoutubeVideoSnippet>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct YoutubeVideoSnippet {
    pub id: YoutubeVideoDetails,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PlaylistYoutubeVideoItem {
    pub contentDetails: YoutubeVideoDetails,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct YoutubeVideoDetails {
    pub videoId: String,
}
