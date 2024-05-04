use crate::{
    domain::database::{add_collection_to_parent, get_collection_id_by_deezer_id},
    handlers::errors::HandlerError,
};

pub async fn add_collection_dependency(
    parent_deezer_id: String,
    child_deezer_id: String,
) -> Result<bool, HandlerError> {
    let parent_id: i32;
    let child_id: i32;
    match get_collection_id_by_deezer_id(parent_deezer_id.clone()) {
        Ok(id) => {
            parent_id = id;
        }
        Err(e) => {
            eprintln!(
                "Error while getting collection id {} : {:?}",
                parent_deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    match get_collection_id_by_deezer_id(child_deezer_id.clone()) {
        Ok(id) => {
            child_id = id;
        }
        Err(e) => {
            eprintln!(
                "Error while getting collection id {} : {:?}",
                child_deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    match add_collection_to_parent(parent_id, child_id) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Error while adding collection {} to collection {}",
                parent_id, child_id
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
    return Ok(true);
}
