use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollection {
    pub name: String,
    pub from_playlist: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionListElement {
    pub name: String,
    pub deezer_id: String,
    pub url: String,
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
    pub deezer_id: String,
    pub title: String,
    pub link: String,
    pub artist: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Collection {
    pub name: String,
    pub deezer_id: String,
    pub url: String,
    pub tracks: Vec<Track>,
    pub children_col: Vec<Collection>,
}
