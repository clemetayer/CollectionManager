use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollection {
    pub name: String,
    pub from_playlist: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionListElement {
    pub name: String
}