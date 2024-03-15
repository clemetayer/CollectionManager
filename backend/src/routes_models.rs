use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollectionInput {
    pub name: String,
    pub modules: [String;2],
    pub from_playlist: String
}