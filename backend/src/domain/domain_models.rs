use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollectionDatabase {
    pub name: String,
    pub deezer_id: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollectionDatabase {
    pub name: String,
}
