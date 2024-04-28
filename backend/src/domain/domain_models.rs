use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollectionDatabase {
    pub name: String,
    pub deezer_id: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionDatabase {
    pub deezer_id: String,
    pub url: String,
    pub name: String,
    pub tracks: Vec<TrackDatabase>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrackDatabase {
    pub deezer_id: String,
    pub title: String,
    pub artist: String,
    pub url: String,
}
