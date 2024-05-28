use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCollectionInput {
    pub name: String,
    pub from_playlist: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddCollectionToParent {
    pub parent_collection_id: String,
    pub child_collection_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoveCollectionToParent {
    pub parent_collection_id: String,
    pub child_collection_id: String,
}
