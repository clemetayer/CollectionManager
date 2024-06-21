use super::collection_commons::{
    convert_string_to_u64, create_collection_from_playlist, get_collection_id_by_deezer_id,
    log_database_error, log_deezer_error,
};
use crate::{
    domain::errors::DomainError,
    infrastructure::{
        database::{
            add_collection_to_parent,
            get_collection_id_by_deezer_id as get_collection_id_by_deezer_id_database,
            remove_collection_to_parent,
        },
        errors::DatabaseError,
    },
};

pub async fn add_collection_dependency(
    parent_deezer_id: &str,
    child_deezer_id: &str,
) -> Result<bool, DomainError> {
    add_collection_if_not_in_database(parent_deezer_id).await?;
    add_collection_if_not_in_database(child_deezer_id).await?;
    add_collection_dependency_to_database(parent_deezer_id, child_deezer_id)?;
    return Ok(true);
}

fn add_collection_dependency_to_database(
    parent_deezer_id: &str,
    child_deezer_id: &str,
) -> Result<bool, DomainError> {
    let parent_id = get_collection_id_by_deezer_id(parent_deezer_id)?;
    let child_id = get_collection_id_by_deezer_id(child_deezer_id)?;
    match add_collection_to_parent(&parent_id, &child_id) {
        Ok(_) => {}
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while adding collection {} to collection {} : {:?}",
                child_id, parent_id, e
            )));
        }
    }
    return Ok(true);
}

async fn add_collection_if_not_in_database(deezer_id: &str) -> Result<bool, DomainError> {
    match get_collection_id_by_deezer_id_database(deezer_id) {
        Ok(_) => {
            return Ok(true);
        } // Collection already exists, non need to add it
        Err(e) => match e {
            DatabaseError::ResultError() => {
                match create_collection_from_playlist(&convert_string_to_u64(deezer_id)).await {
                    Ok(_) => return Ok(true),
                    Err(e) => {
                        return Err(log_deezer_error(&format!(
                            "Error creating collection from playlist id {} : {:?}",
                            deezer_id, e
                        )));
                    }
                }
            }
            DatabaseError::ConnectionError() => {
                return Err(log_database_error(&format!("Connection error while trying to add the collection {} if not in database : {:?}",deezer_id,e)));
            }
        },
    }
}

pub fn remove_collection_dependency(
    parent_deezer_id: &str,
    child_deezer_id: &str,
) -> Result<bool, DomainError> {
    let parent_id = get_collection_id_by_deezer_id(parent_deezer_id)?;
    let child_id = get_collection_id_by_deezer_id(child_deezer_id)?;
    match remove_collection_to_parent(&parent_id, &child_id) {
        Ok(value) => return Ok(value),
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while removing {} from {} : {:?}",
                child_deezer_id, parent_deezer_id, e
            )));
        }
    }
}
