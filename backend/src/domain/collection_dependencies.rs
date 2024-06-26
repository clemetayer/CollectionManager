use super::{
    collection_commons::{
        convert_string_to_u64, create_collection_from_playlist, get_collection_id_by_deezer_id,
        log_database_error, log_deezer_error, log_parameters_error,
    },
    controllers::check_id_valid,
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
    parent_id: &str,
    child_id: &str,
) -> Result<bool, DomainError> {
    log_parameters_error(check_id_valid(parent_id.to_string()))?;
    log_parameters_error(check_id_valid(child_id.to_string()))?;
    add_collection_if_not_in_database(parent_id).await?;
    add_collection_if_not_in_database(child_id).await?;
    add_collection_dependency_to_database(parent_id, child_id)?;
    return Ok(true);
}

fn add_collection_dependency_to_database(
    parent_id: &str,
    child_id: &str,
) -> Result<bool, DomainError> {
    let database_parent_id = get_collection_id_by_deezer_id(parent_id)?;
    let database_child_id = get_collection_id_by_deezer_id(child_id)?;
    match add_collection_to_parent(&database_parent_id, &database_child_id) {
        Ok(_) => {}
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while adding collection {} to collection {} : {:?}",
                database_child_id, database_parent_id, e
            )));
        }
    }
    return Ok(true);
}

async fn add_collection_if_not_in_database(id: &str) -> Result<bool, DomainError> {
    match get_collection_id_by_deezer_id_database(id) {
        Ok(_) => {
            return Ok(true);
        } // Collection already exists, non need to add it
        Err(e) => match e {
            DatabaseError::ResultError() => {
                match create_collection_from_playlist(&convert_string_to_u64(id)).await {
                    Ok(_) => return Ok(true),
                    Err(e) => {
                        return Err(log_deezer_error(&format!(
                            "Error creating collection from playlist id {} : {:?}",
                            id, e
                        )));
                    }
                }
            }
            DatabaseError::ConnectionError() => {
                return Err(log_database_error(&format!("Connection error while trying to add the collection {} if not in database : {:?}",id,e)));
            }
        },
    }
}

pub fn remove_collection_dependency(parent_id: &str, child_id: &str) -> Result<bool, DomainError> {
    log_parameters_error(check_id_valid(parent_id.to_string()))?;
    log_parameters_error(check_id_valid(child_id.to_string()))?;
    let database_parent_id = get_collection_id_by_deezer_id(parent_id)?;
    let database_child_id = get_collection_id_by_deezer_id(child_id)?;
    match remove_collection_to_parent(&database_parent_id, &database_child_id) {
        Ok(value) => return Ok(value),
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while removing {} from {} : {:?}",
                database_child_id, database_parent_id, e
            )));
        }
    }
}
