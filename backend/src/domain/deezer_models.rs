use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreatedPlaylist {
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub id: u64,
    pub title: String,
    pub public: bool,
    pub nb_tracks: u64,
    pub url: String,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub link: String,
    pub artist: String,
}
