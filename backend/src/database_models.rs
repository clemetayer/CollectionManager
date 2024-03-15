use serde::{Deserialize, Serialize};

pub struct Collection {
    pub id: u16,
    pub name: String,
    pub tracks: [Track]
}

pub struct CollectionDependency {
    pub id: u16,
    pub parent: Collection,
    pub children: Collection
}

pub struct Track {
    pub isrc: String, // The International Standard Recording Code, abbreviated to ISRC, is a system of codes that identify audio and music video recordings.
    pub title: String,
    pub artists: [String]
}