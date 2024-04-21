use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollectionInput {
    pub name: String,
    pub from_playlist: Option<String>,
}
